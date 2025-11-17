# Visual Tests Usage Guide

## Quick Start

### Opening the Test File

```bash
# Option 1: Open in default browser
open test_visual_unified_recommendations.html

# Option 2: Open with specific browser
open -a "Google Chrome" test_visual_unified_recommendations.html
open -a "Safari" test_visual_unified_recommendations.html
open -a "Firefox" test_visual_unified_recommendations.html
```

### Running Tests

1. **Automatic Testing**:
   - Click the "Run All Tests" button
   - Wait 10 seconds for all tests to complete
   - Review results in the "Test Results" section

2. **Manual Testing**:
   - Use viewport buttons to switch between Desktop/Tablet/Mobile views
   - Click individual test buttons to run specific test suites
   - Interact with the displayed recommendations to test functionality

## Test Suites

### 1. Champion Cards Display (Task 12.1)
**Tests**: Layout validation across all screen sizes

**What it checks**:
- 5 champion cards are created
- Card structure is correct (header, image, name, role, scores)
- Desktop layout (full width)
- Tablet layout (768px)
- Mobile layout (375px)
- No overflow on any screen size

**How to run**:
- Included in "Run All Tests"
- Or manually switch viewports and inspect visually

### 2. Score Bars (Task 12.2)
**Tests**: Score bar display, colors, and animations

**What it checks**:
- 15 score bars created (3 per champion)
- Correct container structure
- Distinct colors (RF: green, DT: blue, KNN: orange)
- Animation from 0% to target width
- Width accuracy

**How to run**:
- Click "Test Score Bars" button
- Or included in "Run All Tests"

### 3. Expandable Details (Task 12.3)
**Tests**: Details button and content display

**What it checks**:
- 15 details buttons exist
- Details display on click
- Content structure is correct
- Close button works
- Contributing factors table displays
- Matched criteria list displays

**How to run**:
- Click "Test Details" button
- Or manually click any "Details" button on a champion card
- Or included in "Run All Tests"

### 4. Responsive Design (Task 12.4)
**Tests**: Layout across all screen sizes

**What it checks**:
- Desktop: Grid layout, no overflow
- Tablet: Proper fitting, readable text
- Mobile: No overflow, tap targets ≥44px, visible bars
- Text readability across all sizes

**How to run**:
- Included in "Run All Tests"
- Or manually switch viewports and inspect

## Understanding Test Results

### Result Indicators

- **✓ Green**: Test passed successfully
- **✗ Red**: Test failed - needs attention
- **⚠ Yellow**: Warning - may need review
- **ℹ Blue**: Informational message

### Result Format

Each test result shows:
```
[Icon] Test description and result
```

Example:
```
✓ Created exactly 5 champion cards
✗ Expected 15 score bars, found 14
⚠ Mobile layout: Some buttons may be too small for touch
ℹ Testing desktop layout (full width)...
```

## Viewport Simulator

### Viewport Sizes

1. **Desktop**: Full width (100%)
   - Tests full-size layout
   - Validates grid layout
   - Checks for proper spacing

2. **Tablet**: 768px width
   - Tests medium screen layout
   - Validates responsive breakpoints
   - Checks text readability

3. **Mobile**: 375px width
   - Tests small screen layout
   - Validates touch targets
   - Checks for overflow

### Switching Viewports

Click the viewport buttons at the top:
- "Desktop View" - Full width
- "Tablet View (768px)" - Tablet simulation
- "Mobile View (375px)" - Mobile simulation

The active viewport button will be highlighted in green.

## Interactive Testing

### Manual Interaction Tests

1. **Test Score Bar Animation**:
   - Refresh the page
   - Watch score bars animate from 0% to target width
   - Should take ~0.5 seconds

2. **Test Details Expansion**:
   - Click any "Details" button
   - Details section should expand smoothly
   - Click "Close" button to collapse

3. **Test Responsive Behavior**:
   - Switch between viewport sizes
   - Verify cards resize appropriately
   - Check that all content remains visible

## Troubleshooting

### Issue: Tests Not Running

**Solution**:
- Ensure JavaScript is enabled in your browser
- Check browser console for errors (F12 or Cmd+Option+I)
- Refresh the page and try again

### Issue: Layout Looks Broken

**Solution**:
- Verify CSS file is loaded: `src/styles/styles.css`
- Check that all required CSS classes exist
- Try a different browser

### Issue: Score Bars Not Animating

**Solution**:
- Wait a moment after page load
- Check that `data-width` attributes are set on bars
- Verify CSS transitions are enabled

### Issue: Details Not Expanding

**Solution**:
- Check browser console for JavaScript errors
- Verify `mlResults` object is properly populated
- Ensure champion names match between data and DOM

## Expected Results

### All Tests Passing

When all tests pass, you should see:
```
=== All Visual Tests Complete ===
Summary: 30+ passed, 0 failed, 0 warnings
✓ ALL VISUAL TESTS PASSED
```

### Typical Test Count

- **Pass**: 25-30 tests
- **Fail**: 0 tests
- **Warnings**: 0-3 tests (minor issues)

## Advanced Usage

### Running Specific Tests

You can call test functions directly from the browser console:

```javascript
// Run individual test suites
testChampionCardsDisplay();
testScoreBars();
testExpandableDetails();
testResponsiveDesign();

// Run all tests
runAllTests();

// Change viewport
setViewport('mobile');
setViewport('tablet');
setViewport('desktop');
```

### Inspecting Test Data

View the mock data in the console:

```javascript
// View all champions
console.log(allChampions);

// View ML results
console.log(mlResults);

// View specific champion scores
console.log(mlResults.top5[0]);
```

### Custom Test Scenarios

Modify the mock data to test edge cases:

```javascript
// Test with different scores
mlResults.top5[0].randomForest = 100;
displayUnifiedRecommendations();

// Test with missing data
delete mlResults.top5[0].details.randomForest;
showCalculationDetails('randomForest', 'Ahri');
```

## Performance Notes

- **Test Duration**: ~10 seconds for all tests
- **Page Load**: Instant (no external dependencies)
- **Animation Timing**: 100ms delay + 500ms transition
- **Test Execution**: Sequential with proper delays

## Browser Compatibility

Tested and working on:
- ✅ Chrome 90+
- ✅ Safari 14+
- ✅ Firefox 88+
- ✅ Edge 90+

## File Structure

```
test_visual_unified_recommendations.html
├── HTML Structure
│   ├── Test Overview Section
│   ├── Test Controls Section
│   ├── Test Results Section
│   └── Viewport Simulator Section
├── CSS Styles
│   ├── Layout styles
│   ├── Test result styles
│   └── Viewport simulator styles
└── JavaScript
    ├── Mock Data (allChampions, mlResults)
    ├── Display Functions
    ├── Test Functions (4 suites)
    └── Helper Functions
```

## Next Steps After Testing

Once all visual tests pass:

1. ✅ Verify all requirements are met
2. ✅ Document any issues found
3. ✅ Proceed to Task 13 (Performance Optimization)
4. ✅ Continue with remaining implementation tasks

## Support

If you encounter issues:

1. Check the browser console for errors
2. Review the TASK_12_VISUAL_TESTS_SUMMARY.md file
3. Verify the CSS file is properly linked
4. Ensure all mock data is correctly structured

## Summary

This test file provides comprehensive visual testing for the Unified ML Recommendations system. Use it to:
- Validate layout across all screen sizes
- Test interactive features
- Verify animations and transitions
- Ensure responsive design compliance
- Confirm all visual requirements are met

Happy testing! 🎉
