#!/usr/bin/env python3
"""Verify the Rust/Leptos preview flow locally or at a published Pages URL."""

from __future__ import annotations

import argparse
import json
import threading
from datetime import datetime, timezone
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote, urlsplit

from playwright.sync_api import Browser, Page, sync_playwright

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "tests/fixtures/migration/screenshots"
BASE_PATH = "/LOL-Recommender-System"


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


def save_screenshot(page: Page, name: str) -> str:
    page.evaluate("window.scrollTo(0, 0)")
    path = OUTPUT / f"rust-phase6-{name}.png"
    page.screenshot(path=str(path), animations="disabled")
    return str(path.relative_to(ROOT))


def run_user_flow(
    page: Page, viewport_name: str, checks: dict[str, object], observed_dialogs: list[str]
) -> None:
    page.locator("#start-screen").wait_for(state="visible", timeout=15_000)
    checks[f"{viewport_name}_start_screenshot"] = save_screenshot(
        page, f"{viewport_name}-start"
    )

    # Phase 6: Verify Analytics Dashboard integration
    page.get_by_role("button", name="Analytics").click()
    page.locator("#analytics-dashboard").wait_for(state="visible", timeout=5_000)
    checks[f"{viewport_name}_analytics_screenshot"] = save_screenshot(
        page, f"{viewport_name}-analytics"
    )

    # Verify all 5 SVG charts are present and rendered with positive dimensions
    for svg_id in [
        "#heroTypeSvg",
        "#rangeTypeSvg",
        "#resourceTypeSvg",
        "#difficultySvg",
        "#releaseYearSvg",
    ]:
        svg_elem = page.locator(svg_id)
        svg_elem.wait_for(state="visible", timeout=5_000)
        box = svg_elem.bounding_box()
        if not box or box["width"] <= 0 or box["height"] <= 0:
            raise AssertionError(f"SVG chart {svg_id} did not render with positive dimensions: {box}")

    total_champs = page.locator(".analytics-stats-ribbon").text_content()
    if "171" not in (total_champs or ""):
        raise AssertionError(f"Analytics ribbon did not display 171 champions: {total_champs}")
    checks[f"{viewport_name}_analytics_rendered_5_svg_charts"] = True

    # Test "Back to Home"
    page.get_by_role("button", name="← Back to Home").click()
    page.locator("#start-screen").wait_for(state="visible", timeout=5_000)
    checks[f"{viewport_name}_analytics_back_to_home"] = True

    # Test direct URL hash navigation
    current_url = page.url.split("#")[0].rstrip("/") + "/"
    page.goto(f"{current_url}#analytics")
    page.locator("#analytics-dashboard").wait_for(state="visible", timeout=5_000)
    page.get_by_role("button", name="← Back to Home").click()
    page.locator("#start-screen").wait_for(state="visible", timeout=5_000)
    checks[f"{viewport_name}_analytics_hash_routing"] = True

    # Test fallback analytics.html redirect
    page.goto(f"{current_url}analytics.html")
    page.locator("#analytics-dashboard").wait_for(state="visible", timeout=5_000)
    page.get_by_role("button", name="← Back to Home").click()
    page.locator("#start-screen").wait_for(state="visible", timeout=5_000)
    checks[f"{viewport_name}_analytics_html_redirect"] = True

    page.get_by_role("button", name="Find My Champion").click()
    page.locator("#user-registration").wait_for(state="visible", timeout=5_000)
    checks[f"{viewport_name}_registration_screenshot"] = save_screenshot(
        page, f"{viewport_name}-registration"
    )
    if page.evaluate("document.querySelector('#user-form').checkValidity()"):
        raise AssertionError("Registration form should require its legacy required fields")

    page.locator("#fullName").fill("Synthetic Summoner")
    page.locator("#email").fill("synthetic@example.invalid")
    page.locator("#age").fill("25")
    page.locator("#experience").select_option("Intermediate")
    page.locator("#user-form button[type=submit]").click()
    page.locator("#question-1").wait_for(state="visible", timeout=5_000)
    checks[f"{viewport_name}_question_screenshot"] = save_screenshot(
        page, f"{viewport_name}-question-1"
    )

    stored_users = page.evaluate("JSON.parse(localStorage.getItem('championRecommenderUsers') || '[]')")
    if len(stored_users) != 1:
        raise AssertionError(f"Expected 1 user in localStorage, got {len(stored_users)}")
    if stored_users[0].get("fullName") != "Synthetic Summoner" or stored_users[0].get("email") != "synthetic@example.invalid":
        raise AssertionError(f"Stored user mismatch: {stored_users[0]}")
    checks[f"{viewport_name}_stored_users_count"] = len(stored_users)

    first_answer = page.get_by_role("button", name="Medium (4-6)")
    first_answer.focus()
    page.keyboard.press("Enter")
    if first_answer.get_attribute("aria-pressed") != "true":
        raise AssertionError("Question options should support keyboard selection")
    checks[f"{viewport_name}_keyboard_option_selection"] = True
    page.get_by_role("button", name="Next").click()
    page.locator("#question-2").wait_for(state="visible", timeout=5_000)
    page.get_by_role("button", name="Mage").click()
    page.get_by_role("button", name="Next").click()
    page.locator("#question-3").wait_for(state="visible", timeout=5_000)
    mage_positions = page.locator("#question-3 .options .option").all_text_contents()
    if mage_positions != ["Mid", "Support"]:
        raise AssertionError(f"Mage position filter did not match legacy behavior: {mage_positions}")
    checks[f"{viewport_name}_mage_filtered_positions"] = mage_positions
    checks[f"{viewport_name}_filtered_screenshot"] = save_screenshot(
        page, f"{viewport_name}-filtered-question"
    )

    page.get_by_role("button", name="Previous").click()
    page.locator("#question-2").wait_for(state="visible", timeout=5_000)
    page.get_by_role("button", name="Marksman").click()
    page.get_by_role("button", name="Next").click()
    page.locator("#question-3").wait_for(state="visible", timeout=5_000)
    marksman_positions = page.locator("#question-3 .options .option").all_text_contents()
    if marksman_positions != ["Bot"]:
        raise AssertionError(
            f"Marksman position filter did not match legacy behavior: {marksman_positions}"
        )
    page.get_by_role("button", name="Bot").click()
    page.get_by_role("button", name="Next").click()

    question_count = 12
    for _ in range(3, question_count):
        current_question = page.locator(".question-card").get_attribute("id")
        if current_question is None:
            raise AssertionError("The current Rust question card has no ID")
        if current_question == "question-7":
            damage_types = page.locator("#question-7 .options .option").all_text_contents()
            if damage_types != ["Physical", "No Preference"]:
                raise AssertionError(
                    f"Marksman damage-type filter did not match legacy behavior: {damage_types}"
                )
            checks[f"{viewport_name}_marksman_filtered_damage_types"] = damage_types
        page.locator(f"#{current_question} .options .option").first.click()
        next_button = page.locator(f"#{current_question} button.btn-primary")
        next_button.click()

    page.locator("#detailed-analysis").wait_for(state="visible", timeout=15_000)
    recommendation_cards = page.locator("#analysis-content .match-card")
    recommendation_cards.first.wait_for(state="visible", timeout=5_000)
    if recommendation_cards.count() != 10:
        raise AssertionError(
            f"Expected 10 recommendation cards, got {recommendation_cards.count()}"
        )
    if page.locator("#analysis-content .metric-card").count() != 6:
        raise AssertionError("Expected all six existing evaluation metric cards")
    checks[f"{viewport_name}_recommendation_count"] = recommendation_cards.count()
    checks[f"{viewport_name}_top_recommendation"] = recommendation_cards.first.locator(
        ".match-info h4"
    ).inner_text()
    checks[f"{viewport_name}_results_screenshot"] = save_screenshot(
        page, f"{viewport_name}-results"
    )

    recommendation_cards.first.click()
    page.get_by_role("dialog").wait_for(state="visible", timeout=5_000)
    checks[f"{viewport_name}_champion_modal_screenshot"] = save_screenshot(
        page, f"{viewport_name}-champion-modal"
    )
    page.keyboard.press("Escape")
    page.get_by_role("dialog").wait_for(state="hidden", timeout=5_000)

    page.locator("#analysis-content .metric-card").first.click()
    page.get_by_role("dialog").wait_for(state="visible", timeout=5_000)
    checks[f"{viewport_name}_metric_modal_screenshot"] = save_screenshot(
        page, f"{viewport_name}-metric-modal"
    )
    page.keyboard.press("Escape")
    page.get_by_role("dialog").wait_for(state="hidden", timeout=5_000)

    recent_champions = page.evaluate("JSON.parse(localStorage.getItem('recentChampions') || '[]')")
    if len(recent_champions) != 10:
        raise AssertionError(f"Expected 10 recent champions in localStorage, got {len(recent_champions)}")
    checks[f"{viewport_name}_recent_champions_count"] = len(recent_champions)
    checks[f"{viewport_name}_local_storage_keys"] = page.evaluate("Object.keys(localStorage)")

    # Test "Save" button and saved users modal
    page.get_by_role("button", name="Save").click()
    page.locator(".modal.active").wait_for(state="visible", timeout=5_000)
    checks[f"{viewport_name}_saved_users_modal_screenshot"] = save_screenshot(
        page, f"{viewport_name}-saved-users-modal"
    )
    page.locator(".modal-footer button").first.scroll_into_view_if_needed()
    with page.expect_download() as download_info:
        page.locator(".modal-footer button").first.click()
    download = download_info.value
    if download.suggested_filename != "champion_recommendations.csv":
        raise AssertionError(f"Unexpected table export filename: {download.suggested_filename}")
    checks[f"{viewport_name}_table_csv_download"] = download.suggested_filename

    page.locator(".modal-footer button").last.scroll_into_view_if_needed()
    page.locator(".modal-footer button").last.click()
    page.locator(".modal").wait_for(state="hidden", timeout=5_000)

    # Test Ctrl+Shift+E shortcut
    with page.expect_download() as download_info:
        page.keyboard.press("Control+Shift+E")
    download = download_info.value
    if download.suggested_filename != "champion_recommender_users.csv":
        raise AssertionError(f"Unexpected user export filename: {download.suggested_filename}")
    checks[f"{viewport_name}_users_csv_download"] = download.suggested_filename

    # Test Ctrl+Shift+A admin shortcut
    page.keyboard.press("Control+Shift+A")
    page.wait_for_timeout(500)
    admin_dialogs = [msg for msg in observed_dialogs if "Enhanced Admin Panel" in msg]
    if not admin_dialogs:
        raise AssertionError("Admin dialog not triggered")
    checks[f"{viewport_name}_admin_dialog_triggered"] = True

    page.get_by_role("button", name="Try Again").click()
    page.locator("#start-screen").wait_for(state="visible", timeout=5_000)
    checks[f"{viewport_name}_restart_returns_to_start"] = True
    page.get_by_role("button", name="Find My Champion").click()
    if page.locator("#fullName").input_value() != "Synthetic Summoner":
        raise AssertionError("Restart should retain the entered registration values")
    checks[f"{viewport_name}_restart_retains_registration"] = True


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--url",
        help="Preview URL to verify; defaults to the locally built project-subpath preview.",
    )
    args = parser.parse_args()

    server = None
    server_thread = None
    if args.url:
        preview_url = args.url.rstrip("/") + "/"
        output_name = "rust-phase6-published"
    else:
        server = ThreadingHTTPServer(("127.0.0.1", 0), PagesHandler)
        server_thread = threading.Thread(target=server.serve_forever, daemon=True)
        server_thread.start()
        preview_url = f"http://127.0.0.1:{server.server_port}{BASE_PATH}/rust-preview/"
        output_name = "rust-phase6-local"

    requests: list[dict[str, str]] = []
    responses: list[dict[str, object]] = []
    failures: list[dict[str, str]] = []
    blocked: list[dict[str, str]] = []
    page_errors: list[str] = []
    console_errors: list[str] = []
    checks: dict[str, object] = {}
    report: dict[str, object] = {
        "captured_at_utc": datetime.now(timezone.utc).isoformat(),
        "preview_url": preview_url,
        "browser": {},
        "checks": checks,
        "requests": requests,
        "responses": responses,
        "blocked_requests": blocked,
        "request_failures": failures,
        "page_errors": page_errors,
        "console_errors": console_errors,
        "notes": [
            "Each viewport used a fresh non-persistent browser context and synthetic form values.",
            "Registration persists to championRecommenderUsers and recommendations update recentChampions in localStorage.",
            "Save button displays saved records table and supports CSV export; Ctrl+Shift+E exports users and Ctrl+Shift+A opens admin dialog.",
            "Gemini requests and external non-GET requests are blocked; URLs omit query strings and bodies.",
        ],
    }

    def record_request(request) -> None:
        parsed = urlsplit(request.url)
        requests.append(
            {
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
            blocked.append({"host": host, "path": parsed.path, "reason": "AI endpoint blocked"})
            route.abort("blockedbyclient")
        elif host not in {
            "127.0.0.1",
            "localhost",
            "abdullah-binmadhi.github.io",
        } and request.method != "GET":
            blocked.append(
                {"host": host, "path": parsed.path, "reason": "external non-GET blocked"}
            )
            route.abort("blockedbyclient")
        else:
            route.continue_()

    try:
        OUTPUT.mkdir(parents=True, exist_ok=True)
        with sync_playwright() as playwright:
            browser = playwright.chromium.launch(headless=True)
            report["browser"] = {"name": "Chromium", "version": browser.version}
            for viewport_name, viewport in (
                ("desktop", {"width": 1440, "height": 900}),
                ("mobile", {"width": 390, "height": 844}),
            ):
                context = browser.new_context(viewport=viewport, device_scale_factor=1)
                page = context.new_page()
                page.on("request", record_request)
                page.on("response", record_response)
                page.on(
                    "requestfailed",
                    lambda request: failures.append(
                        {
                            "host": urlsplit(request.url).hostname or "",
                            "path": urlsplit(request.url).path,
                            "failure": request.failure or "unknown",
                        }
                    ),
                )
                page.on("pageerror", lambda error: page_errors.append(str(error)))
                page.on(
                    "console",
                    lambda message: console_errors.append(message.text)
                    if message.type == "error"
                    else None,
                )
                observed_dialogs: list[str] = []
                def on_dialog(dialog):
                    observed_dialogs.append(dialog.message)
                    if "Enhanced Admin Panel" in dialog.message:
                        dialog.dismiss()
                    else:
                        dialog.accept()

                page.on("dialog", on_dialog)
                context.route("**/*", guard_request)

                response = page.goto(preview_url, wait_until="domcontentloaded", timeout=60_000)
                if response is None or not response.ok:
                    raise AssertionError(
                        f"Preview did not return a successful document response: {response}"
                    )
                page.locator("#start-screen").wait_for(state="visible", timeout=15_000)

                stylesheets = page.locator('link[rel="stylesheet"]').evaluate_all(
                    "links => links.map(link => ({href: new URL(link.href).pathname, loaded: link.sheet !== null}))"
                )
                css_responses = [
                    item
                    for item in responses
                    if item["path"].endswith(".css") and item["status"] == 200
                ]
                if not stylesheets or not all(item["loaded"] for item in stylesheets) or not css_responses:
                    raise AssertionError(
                        f"Preview stylesheet failed to load: {stylesheets}; responses={css_responses}"
                    )
                checks[f"{viewport_name}_stylesheet_links"] = stylesheets
                checks[f"{viewport_name}_document_status"] = response.status
                checks[f"{viewport_name}_horizontal_overflow"] = page.evaluate(
                    "document.documentElement.scrollWidth > window.innerWidth"
                )
                if checks[f"{viewport_name}_horizontal_overflow"]:
                    raise AssertionError(f"Horizontal overflow at {viewport_name} viewport")

                run_user_flow(page, viewport_name, checks, observed_dialogs)
                if page_errors:
                    raise AssertionError(f"Browser runtime errors occurred: {page_errors}")
                if blocked:
                    raise AssertionError(f"Unexpected requests were blocked by the safety guard: {blocked}")
                context.close()
            browser.close()

        wasm_responses = [
            item for item in responses if ".wasm" in item["path"] and item["status"] == 200
        ]
        if not wasm_responses:
            raise AssertionError(f"No successful WebAssembly asset request observed: {responses}")
        checks["successful_wasm_responses"] = wasm_responses
        checks["page_errors"] = page_errors
        checks["console_errors"] = console_errors
    finally:
        if server is not None:
            server.shutdown()
            server.server_close()
        if server_thread is not None:
            server_thread.join(timeout=5)

    report_path = OUTPUT / f"{output_name}-observations.json"
    report_path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(
        json.dumps(
            {
                "preview_url": preview_url,
                "browser": report["browser"],
                "checks": checks,
                "report": str(report_path.relative_to(ROOT)),
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
