#!/usr/bin/env python3
"""Capture the Phase 0 website baseline using isolated Playwright contexts."""

from __future__ import annotations

import json
import subprocess
import threading
from datetime import datetime, timezone
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote, urlsplit

from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "tests/fixtures/migration/screenshots"
BASE_PATH = "/LOL-Recommender-System"
FAKE_EMAIL = "phase0-test@example.invalid"
VIEWPORTS = {
    "desktop": {"width": 1440, "height": 900},
    "mobile": {"width": 390, "height": 844},
}


class PagesHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(ROOT), **kwargs)

    def translate_path(self, path: str) -> str:
        local_path = unquote(urlsplit(path).path)
        if local_path == BASE_PATH or local_path == BASE_PATH + "/":
            local_path = "/"
        elif local_path.startswith(BASE_PATH + "/"):
            local_path = local_path[len(BASE_PATH) :]
        return super().translate_path(local_path)

    def log_message(self, _format: str, *_args) -> None:
        pass


def git_state() -> dict[str, object]:
    commit = subprocess.run(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, check=True, capture_output=True, text=True
    ).stdout.strip()
    status = subprocess.run(
        ["git", "status", "--porcelain"], cwd=ROOT, check=True, capture_output=True, text=True
    ).stdout
    return {"commit": commit, "working_tree_dirty": bool(status.strip())}


def screenshot(page, name: str) -> str:
    page.evaluate("window.scrollTo(0, 0)")
    page.wait_for_timeout(150)
    path = OUTPUT / name
    page.screenshot(path=str(path), animations="disabled")
    return path.name


def wait_for_images(page, selector: str) -> list[dict[str, object]]:
    page.wait_for_function(
        "selector => [...document.querySelectorAll(selector)].every(image => image.complete)",
        arg=selector,
        timeout=20_000,
    )
    return page.locator(selector).evaluate_all(
        "images => images.map(image => ({loaded: image.naturalWidth > 0, host: new URL(image.src).hostname, path: new URL(image.src).pathname}))"
    )


def capture_viewport(browser, label: str, viewport: dict[str, int], base_url: str) -> dict:
    context = browser.new_context(
        viewport=viewport,
        device_scale_factor=1,
        accept_downloads=True,
    )
    requests: list[dict[str, str]] = []
    responses: list[dict[str, object]] = []
    failures: list[dict[str, str]] = []
    blocked: list[dict[str, str]] = []
    page_errors: list[str] = []
    console_errors: list[str] = []
    stage = ["initial-load"]

    def record_request(request) -> None:
        parsed = urlsplit(request.url)
        requests.append(
            {
                "stage": stage[0],
                "method": request.method,
                "host": parsed.hostname or "",
                "path": parsed.path,
                "resource_type": request.resource_type,
            }
        )

    def record_response(response) -> None:
        parsed = urlsplit(response.url)
        responses.append(
            {
                "stage": stage[0],
                "method": response.request.method,
                "host": parsed.hostname or "",
                "path": parsed.path,
                "status": response.status,
            }
        )

    def guard_request(route) -> None:
        request = route.request
        parsed = urlsplit(request.url)
        host = (parsed.hostname or "").lower()
        if host == "generativelanguage.googleapis.com":
            blocked.append({"stage": stage[0], "host": host, "path": parsed.path, "reason": "AI endpoint blocked"})
            route.abort("blockedbyclient")
            return
        if host not in {"127.0.0.1", "localhost"} and request.method != "GET":
            blocked.append({"stage": stage[0], "host": host, "path": parsed.path, "reason": "external non-GET blocked"})
            route.abort("blockedbyclient")
            return
        route.continue_()

    page = context.new_page()
    page.on("request", record_request)
    page.on("response", record_response)
    page.on(
        "requestfailed",
        lambda request: failures.append(
            {
                "stage": stage[0],
                "host": urlsplit(request.url).hostname or "",
                "path": urlsplit(request.url).path,
                "failure": request.failure or "unknown",
            }
        ),
    )
    page.on("pageerror", lambda error: page_errors.append(str(error)))
    page.on("console", lambda message: console_errors.append(message.text) if message.type == "error" else None)
    context.route("**/*", guard_request)

    shots: list[str] = []
    checks: dict[str, object] = {"viewport": viewport, "isolated_storage_before": []}
    root_url = f"{base_url}{BASE_PATH}/index.html"
    app_url = f"{base_url}{BASE_PATH}/src/index.html"

    try:
        stage[0] = "root-entry"
        page.goto(root_url, wait_until="domcontentloaded", timeout=30_000)
        page.locator("body h1").wait_for(state="visible", timeout=15_000)
        checks["isolated_storage_before"] = page.evaluate("Object.keys(localStorage)")
        if checks["isolated_storage_before"]:
            raise AssertionError("Fresh browser context unexpectedly contained localStorage data")
        shots.append(screenshot(page, f"{label}-01-root-entry.png"))

        stage[0] = "navigation-to-recommender"
        page.get_by_role("link", name="Find My Champion", exact=False).click()
        page.wait_for_url("**/src/index.html", timeout=10_000)
        page.locator("#start-screen").wait_for(state="visible", timeout=15_000)
        checks["start_images"] = wait_for_images(page, "#start-screen .champion-avatar")
        shots.append(screenshot(page, f"{label}-02-recommender-start.png"))

        stage[0] = "registration"
        page.locator("#start-screen .btn-hero").first.click()
        page.locator("#user-registration.active").wait_for(state="visible", timeout=5_000)
        shots.append(screenshot(page, f"{label}-03-registration.png"))
        page.locator("#fullName").fill("Synthetic Phase 0 Tester")
        page.locator("#email").fill(FAKE_EMAIL)
        page.locator("#age").fill("27")
        page.locator("#experience").select_option(label="Intermediate")
        page.locator("#user-form button[type='submit']").click()
        page.locator("#question-1.active .option").first.wait_for(state="visible", timeout=15_000)

        stage[0] = "questionnaire"
        shots.append(screenshot(page, f"{label}-04-question-1.png"))
        page.locator("#question-1 .option", has_text="Medium (4-6)").first.click()
        page.locator("#next-1").click()
        page.locator("#question-2.active .option").first.wait_for(state="visible", timeout=5_000)
        page.locator("#question-2 .option", has_text="Mage").first.click()
        page.locator("#next-2").click()
        page.locator("#question-3.active .option").first.wait_for(state="visible", timeout=5_000)
        filtered_options = [
            " ".join(text.split()) for text in page.locator("#question-3 .option").all_text_contents()
        ]
        if len(filtered_options) != 2:
            raise AssertionError(f"Mage role should filter position choices to two options; got {filtered_options}")
        checks["mage_position_options"] = filtered_options
        shots.append(screenshot(page, f"{label}-05-filtered-question.png"))

        stage[0] = "recommendation-scoring"
        for question_id in range(3, 13):
            page.locator(f"#question-{question_id}.active .option").first.click()
            page.locator(f"#next-{question_id}").click()
            if question_id < 12:
                page.locator(f"#question-{question_id + 1}.active .option").first.wait_for(
                    state="visible", timeout=5_000
                )
        page.locator("#detailed-analysis.active").wait_for(state="visible", timeout=30_000)
        page.locator("#analysis-content .match-card").first.wait_for(state="visible", timeout=15_000)
        result_count = page.locator("#analysis-content .match-card").count()
        checks["recommendation_card_count"] = result_count
        if result_count == 0:
            raise AssertionError("Scoring completed without rendering recommendation cards")
        checks["recommendation_images"] = wait_for_images(page, "#analysis-content .match-champion-img")
        shots.append(screenshot(page, f"{label}-06-recommendations.png"))

        stage[0] = "metric-modal"
        page.locator(".metric-card-p1").click()
        page.locator("#metric-modal.active").wait_for(state="visible", timeout=5_000)
        shots.append(screenshot(page, f"{label}-07-metric-modal.png"))
        page.keyboard.press("Escape")
        page.locator("#metric-modal.active").wait_for(state="hidden", timeout=5_000)

        stage[0] = "champion-modal"
        page.locator("#analysis-content .match-card").first.click()
        page.locator("#champion-modal.active").wait_for(state="visible", timeout=5_000)
        checks["champion_modal_name"] = page.locator("#champion-modal-name").inner_text()
        checks["champion_modal_image"] = wait_for_images(page, "#champion-modal-img")
        shots.append(screenshot(page, f"{label}-08-champion-modal.png"))
        page.keyboard.press("Escape")
        page.locator("#champion-modal.active").wait_for(state="hidden", timeout=5_000)

        stage[0] = "storage-and-csv-export"
        stored = page.evaluate(
            """() => ({
                users: JSON.parse(localStorage.getItem('championRecommenderUsers') || '[]'),
                recent: JSON.parse(localStorage.getItem('recentChampions') || '[]'),
                keys: Object.keys(localStorage)
            })"""
        )
        checks["storage"] = {
            "keys": stored["keys"],
            "synthetic_registration_count": len(stored["users"]),
            "synthetic_record_matches": len(stored["users"]) == 1
            and stored["users"][0].get("email") == FAKE_EMAIL,
            "recent_champion_count": len(stored["recent"]),
        }
        if not checks["storage"]["synthetic_record_matches"] or not stored["recent"]:
            raise AssertionError("Expected synthetic registration and recent champions in isolated storage")

        page.on("dialog", lambda dialog: dialog.accept())
        with page.expect_download(timeout=10_000) as download_info:
            page.keyboard.press("Control+Shift+E")
        download = download_info.value
        checks["csv_export_filename"] = download.suggested_filename
        if download.suggested_filename != "champion_recommender_users.csv":
            raise AssertionError(f"Unexpected CSV export filename: {download.suggested_filename}")

        stage[0] = "analytics-navigation"
        page.goto(root_url, wait_until="domcontentloaded", timeout=30_000)
        page.get_by_role("link", name="Analytics Dashboard", exact=False).click()
        page.wait_for_url("**/src/analytics/analytics.html", timeout=15_000)
        page.locator("#heroTypeChart").wait_for(state="visible", timeout=10_000)
        page.wait_for_timeout(2_000)
        checks["analytics_chart_count"] = page.locator(".chart-container canvas").count()
        shots.append(screenshot(page, f"{label}-09-analytics.png"))

        checks["screenshots"] = shots
        checks["external_ai_requests_attempted"] = len(
            [item for item in blocked if item["host"] == "generativelanguage.googleapis.com"]
        )
        if checks["external_ai_requests_attempted"]:
            raise AssertionError("A Gemini request was attempted; it was blocked by the capture guard")
        return {
            "checks": checks,
            "requests": requests,
            "responses": responses,
            "blocked_requests": blocked,
            "request_failures": failures,
            "page_errors": page_errors,
            "console_errors": console_errors,
        }
    finally:
        context.close()


def main() -> None:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    server = ThreadingHTTPServer(("127.0.0.1", 0), PagesHandler)
    server_thread = threading.Thread(target=server.serve_forever, daemon=True)
    server_thread.start()
    base_url = f"http://127.0.0.1:{server.server_port}"

    result = {
        "captured_at_utc": datetime.now(timezone.utc).isoformat(),
        "source_state": git_state(),
        "browser": {},
        "viewports": {},
        "notes": [
            "Each viewport used a new non-persistent Playwright browser context.",
            "Registration values were synthetic and localStorage was discarded when the context closed.",
            "Gemini endpoints and external non-GET requests were blocked; request URLs omit query strings and bodies.",
            "The CSV download was observed but not retained.",
        ],
    }

    try:
        with sync_playwright() as playwright:
            browser = playwright.chromium.launch(headless=True)
            result["browser"] = {"name": "Chromium", "version": browser.version}
            for label, viewport in VIEWPORTS.items():
                result["viewports"][label] = capture_viewport(browser, label, viewport, base_url)
            browser.close()
    finally:
        server.shutdown()
        server.server_close()
        server_thread.join(timeout=5)

    report_path = OUTPUT / "browser-observations.json"
    report_path.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"browser": result["browser"], "viewports": {
        name: {
            "screenshots": data["checks"]["screenshots"],
            "recommendations": data["checks"]["recommendation_card_count"],
            "filtered_options": data["checks"]["mage_position_options"],
            "storage": data["checks"]["storage"],
            "csv_export": data["checks"]["csv_export_filename"],
            "analytics_charts": data["checks"]["analytics_chart_count"],
            "request_count": len(data["requests"]),
            "blocked_request_count": len(data["blocked_requests"]),
            "page_errors": data["page_errors"],
        }
        for name, data in result["viewports"].items()
    }, "report": str(report_path.relative_to(ROOT))}, indent=2))


if __name__ == "__main__":
    main()
