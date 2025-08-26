// Winrate Data Updater for LoL Champion Recommender
// This script helps manage and update champion statistics

class WinrateDataManager {
    constructor() {
        this.currentPatch = '14.23';
        this.lastUpdated = new Date().toISOString().split('T')[0];
    }

    // Generate stats object for a champion
    generateChampionStats(winRate, pickRate, banRate, tier = 'B', trend = 'stable') {
        return {
            winRate: parseFloat(winRate.toFixed(1)),
            pickRate: parseFloat(pickRate.toFixed(1)),
            banRate: parseFloat(banRate.toFixed(1)),
            tier: tier,
            trend: trend, // 'rising', 'falling', 'stable'
            patch: this.currentPatch,
            lastUpdated: this.lastUpdated
        };
    }

    // Determine tier based on winrate and other factors
    calculateTier(winRate, pickRate, banRate) {
        // S Tier: High winrate + decent pickrate OR very high winrate
        if ((winRate >= 53 && pickRate >= 3) || winRate >= 55) return 'S';
        
        // A Tier: Good winrate with reasonable pickrate
        if ((winRate >= 51.5 && pickRate >= 2) || (winRate >= 52.5)) return 'A';
        
        // B Tier: Average performance
        if (winRate >= 49.5 && winRate < 51.5) return 'B';
        
        // C Tier: Below average
        if (winRate >= 47 && winRate < 49.5) return 'C';
        
        // D Tier: Poor performance
        return 'D';
    }

    // Determine trend based on recent performance (this would need historical data)
    calculateTrend(currentWinRate, previousWinRate = null) {
        if (!previousWinRate) return 'stable';
        
        const difference = currentWinRate - previousWinRate;
        if (difference >= 1) return 'rising';
        if (difference <= -1) return 'falling';
        return 'stable';
    }

    // Sample data for popular champions (based on recent patch data)
    getSampleChampionStats() {
        return {
            // Top Tier Performers
            'Evelynn': this.generateChampionStats(53.2, 3.9, 12.7, 'A+', 'rising'),
            'Garen': this.generateChampionStats(53.1, 6.8, 2.3, 'A+', 'rising'),
            'Lux': this.generateChampionStats(52.3, 9.1, 3.7, 'A', 'stable'),
            'Ahri': this.generateChampionStats(52.1, 8.4, 2.1, 'A', 'stable'),
            'Jinx': this.generateChampionStats(51.8, 12.3, 4.2, 'A', 'stable'),
            'Diana': this.generateChampionStats(51.7, 4.1, 3.8, 'A', 'rising'),
            
            // Mid Tier
            'Ashe': this.generateChampionStats(50.9, 8.7, 1.8, 'B+', 'rising'),
            'Ekko': this.generateChampionStats(50.9, 5.7, 8.2, 'B+', 'stable'),
            'Thresh': this.generateChampionStats(50.2, 11.4, 6.8, 'B+', 'stable'),
            'Ezreal': this.generateChampionStats(49.8, 18.2, 3.1, 'B', 'stable'),
            'Akali': this.generateChampionStats(49.8, 6.2, 15.3, 'B', 'falling'),
            'Zed': this.generateChampionStats(49.5, 7.8, 12.4, 'B', 'stable'),
            
            // Lower Tier
            'Yasuo': this.generateChampionStats(49.2, 15.6, 28.4, 'B', 'falling'),
            'Master Yi': this.generateChampionStats(48.9, 8.3, 5.7, 'C+', 'falling'),
            'Leona': this.generateChampionStats(48.7, 9.2, 4.1, 'C+', 'stable'),
            'Annie': this.generateChampionStats(48.4, 3.2, 1.9, 'C', 'falling'),
            'Darius': this.generateChampionStats(47.8, 5.4, 7.2, 'C', 'falling'),
            'Soraka': this.generateChampionStats(47.2, 6.1, 2.8, 'C', 'falling')
        };
    }

    // Generate JavaScript code to update champion database
    generateUpdateCode(championStats) {
        let code = '// Champion Stats Update - Patch ' + this.currentPatch + '\n';
        code += '// Generated on: ' + this.lastUpdated + '\n\n';
        
        for (const [championName, stats] of Object.entries(championStats)) {
            code += `// ${championName}\n`;
            code += `stats: ${JSON.stringify(stats, null, 4)},\n\n`;
        }
        
        return code;
    }

    // Validate stats data
    validateStats(stats) {
        const errors = [];
        
        if (stats.winRate < 0 || stats.winRate > 100) {
            errors.push('Win rate must be between 0 and 100');
        }
        
        if (stats.pickRate < 0 || stats.pickRate > 100) {
            errors.push('Pick rate must be between 0 and 100');
        }
        
        if (stats.banRate < 0 || stats.banRate > 100) {
            errors.push('Ban rate must be between 0 and 100');
        }
        
        if (!['S', 'A+', 'A', 'B+', 'B', 'C+', 'C', 'D'].includes(stats.tier)) {
            errors.push('Invalid tier. Must be S, A+, A, B+, B, C+, C, or D');
        }
        
        if (!['rising', 'falling', 'stable'].includes(stats.trend)) {
            errors.push('Invalid trend. Must be rising, falling, or stable');
        }
        
        return errors;
    }

    // Format stats for display
    formatStatsForDisplay(stats) {
        return {
            winRate: `${stats.winRate}%`,
            pickRate: `${stats.pickRate}%`,
            banRate: `${stats.banRate}%`,
            tier: stats.tier,
            trend: stats.trend,
            trendIcon: this.getTrendIcon(stats.trend),
            patch: stats.patch,
            lastUpdated: stats.lastUpdated
        };
    }

    getTrendIcon(trend) {
        switch(trend) {
            case 'rising': return '📈';
            case 'falling': return '📉';
            case 'stable': return '➡️';
            default: return '';
        }
    }
}

// Usage example
const dataManager = new WinrateDataManager();
const sampleStats = dataManager.getSampleChampionStats();

console.log('=== Champion Stats Update ===');
console.log(dataManager.generateUpdateCode(sampleStats));

console.log('=== Sample Champion Stats ===');
Object.entries(sampleStats).forEach(([name, stats]) => {
    console.log(`${name}: ${JSON.stringify(dataManager.formatStatsForDisplay(stats), null, 2)}`);
});

// Export for use in other files
if (typeof module !== 'undefined' && module.exports) {
    module.exports = WinrateDataManager;
}

// Instructions for updating data:
console.log(`
=== How to Update Champion Stats ===

1. MANUAL UPDATE (Current Method):
   - Edit the champion database files directly
   - Add stats object to each champion
   - Update patch version and date

2. SEMI-AUTOMATED (Recommended):
   - Use this script to generate stats objects
   - Copy-paste into champion database
   - Validate data before deployment

3. FUTURE AUTOMATION:
   - Set up backend service to fetch from APIs
   - Schedule regular updates
   - Implement data validation and caching

Data Sources:
- U.GG: https://u.gg/lol/champions
- OP.GG: https://op.gg/champions
- LoLalytics: https://lolalytics.com/lol/tierlist/
- Champion.gg: https://champion.gg/

Update Frequency:
- Major patches: Full update
- Minor patches: Selective updates
- Hotfixes: Affected champions only
`);