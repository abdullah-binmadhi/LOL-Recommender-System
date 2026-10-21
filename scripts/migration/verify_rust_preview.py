#!/usr/bin/env python3
"""Verify the Rust preview locally or at a published GitHub Pages URL."""

from __future__ import annotations

import argparse
import json
import threading
from datetime import datetime, timezone
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote, urlsplit

from playwright.sync_api import sync_playwright

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
        output_name = "rust-preview-published"
    else:
        server = ThreadingHTTPServer(("127.0.0.1", 0), PagesHandler)
        server_thread = threading.Thread(target=server.serve_forever, daemon=True)
        server_thread.start()
        preview_url = f"http://127.0.0.1:{server.server_port}{BASE_PATH}/rust-preview/"
        output_name = "rust-preview-local"

    requests: list[dict[str, str]] = []
    responses: list[dict[str, object]] = []
    failures: list[dict[str, str]] = []
    blocked: list[dict[str, str]] = []
    page_errors: list[str] = []
    console_errors: list[str] = []

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
        elif host not in {"127.0.0.1", "localhost", "abdullah-binmadhi.github.io"} and request.method != "GET":
            blocked.append({"host": host, "path": parsed.path, "reason": "external non-GET blocked"})
            route.abort("blockedbyclient")
        else:
            route.continue_()

    result: dict[str, object] = {
        "captured_at_utc": datetime.now(timezone.utc).isoformat(),
        "preview_url": preview_url,
        "browser": {},
        "checks": {},
        "requests": requests,
        "responses": responses,
        "blocked_requests": blocked,
        "request_failures": failures,
        "page_errors": page_errors,
        "console_errors": console_errors,
        "notes": [
            "A fresh non-persistent browser context was used; no registration or user data was entered.",
            "Question selections were synthetic and held in Rust UI state only.",
            "Gemini endpoints and external non-GET requests were blocked; request URLs omit query strings and bodies.",
        ],
    }

    try:
        with sync_playwright() as playwright:
            browser = playwright.chromium.launch(headless=True)
            result["browser"] = {"name": "Chromium", "version": browser.version}
            context = browser.new_context(viewport={"width": 1440, "height": 900}, device_scale_factor=1)
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
            page.on("console", lambda message: console_errors.append(message.text) if message.type == "error" else None)
            context.route("**/*", guard_request)

            response = page.goto(preview_url, wait_until="networkidle", timeout=60_000)
            if response is None or not response.ok:
                raise AssertionError(f"Preview did not return a successful document response: {response}")
            page.locator("main.page-shell").wait_for(state="visible", timeout=15_000)
            page.locator("fieldset.question-card").first.wait_for(state="visible", timeout=15_000)

            stylesheets = page.locator('link[rel="stylesheet"]').evaluate_all(
                "links => links.map(link => ({href: new URL(link.href).pathname, loaded: link.sheet !== null}))"
            )
            css_responses = [item for item in responses if item["path"].endswith(".css") and item["status"] == 200]
            if not stylesheets or not all(item["loaded"] for item in stylesheets) or not css_responses:
                raise AssertionError(f"Preview stylesheet failed to load: {stylesheets}; responses={css_responses}")

            css_computed = page.locator("main.page-shell").evaluate(
                "element => ({display: getComputedStyle(element).display, maxWidth: getComputedStyle(element).maxWidth})"
            )
            if css_computed["display"] == "inline":
                raise AssertionError(f"Preview CSS did not apply to the page shell: {css_computed}")

            question_count = page.locator("fieldset.question-card").count()
            if question_count != 12:
                raise AssertionError(f"Expected 12 Rust-rendered questions, got {question_count}")
            if page.locator("button.primary-button").is_enabled():
                raise AssertionError("Rust recommendations should remain disabled until all questions are answered")

            screenshot_path = OUTPUT / f"{output_name}.png"
            page.screenshot(path=str(screenshot_path), animations="disabled")
            if page.evaluate("Object.keys(localStorage)"):
                raise AssertionError("Rust preview unexpectedly wrote localStorage before answering")

            for index in range(question_count):
                page.locator("fieldset.question-card").nth(index).locator('input[type="radio"]').first.check()
            answered_text = page.locator(".toolbar").inner_text()
            if f"{question_count} / {question_count}" not in answered_text:
                raise AssertionError(f"Questionnaire answer counter did not reach completion: {answered_text}")
            if not page.locator("button.primary-button").is_enabled():
                raise AssertionError("Rust recommendation button did not enable after all questions were answered")

            page.get_by_role("button", name="Run Rust recommendations").click()
            result_pre = page.locator("section.results pre")
            result_pre.wait_for(state="visible", timeout=15_000)
            result_text = result_pre.inner_text()
            recommendation_lines = [line for line in result_text.splitlines() if line.strip()[:1].isdigit() and ". " in line]
            if len(recommendation_lines) != 10 or "Rust recommendations" not in result_text:
                raise AssertionError(f"Expected 10 Rust recommendations, got {len(recommendation_lines)}")
            if page.evaluate("Object.keys(localStorage)"):
                raise AssertionError("Rust preview must not persist answers or recommendations")

            wasm_responses = [item for item in responses if ".wasm" in item["path"] and item["status"] == 200]
            if not wasm_responses:
                raise AssertionError(f"No successful WebAssembly asset request observed: {responses}")
            if blocked:
                raise AssertionError(f"Unexpected requests were blocked by the safety guard: {blocked}")
            if page_errors:
                raise AssertionError(f"Browser runtime errors occurred: {page_errors}")

            result["checks"] = {
                "document_status": response.status,
                "stylesheet_links": stylesheets,
                "successful_css_responses": css_responses,
                "computed_page_shell_style": css_computed,
                "question_count": question_count,
                "questionnaire_answer_counter": "12 / 12",
                "recommendation_count": len(recommendation_lines),
                "top_recommendations": recommendation_lines,
                "successful_wasm_responses": wasm_responses,
                "local_storage_keys_after_scoring": page.evaluate("Object.keys(localStorage)"),
                "screenshot": str(screenshot_path.relative_to(ROOT)),
            }
            context.close()
            browser.close()
    finally:
        if server is not None:
            server.shutdown()
            server.server_close()
        if server_thread is not None:
            server_thread.join(timeout=5)

    report_path = OUTPUT / f"{output_name}-observations.json"
    report_path.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"preview_url": preview_url, "browser": result["browser"], "checks": result["checks"], "report": str(report_path.relative_to(ROOT))}, indent=2))


if __name__ == "__main__":
    main()
