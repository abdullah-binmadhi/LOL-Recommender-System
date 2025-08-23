// Utility script to add champion images to your champion database
// This script helps generate image URLs for all League of Legends champions

// Function to generate image URLs for a champion
function generateChampionImageUrls(championName) {
    // Convert champion name to the format used by Riot's API
    // Remove spaces, apostrophes, and special characters
    const apiName = championName
        .replace(/\s+/g, '')           // Remove spaces
        .replace(/'/g, '')             // Remove apostrophes
        .replace(/\./g, '')            // Remove dots
        .replace(/&/g, '')             // Remove ampersands
        .replace(/-/g, '');            // Remove hyphens
    
    return {
        square: `https://ddragon.leagueoflegends.com/cdn/14.1.1/img/champion/${apiName}.png`,
        splash: `https://ddragon.leagueoflegends.com/cdn/img/champion/splash/${apiName}_0.jpg`,
        loading: `https://ddragon.leagueoflegends.com/cdn/img/champion/loading/${apiName}_0.jpg`
    };
}

// Special cases for champions with different API names
const championNameMappings = {
    'Wukong': 'MonkeyKing',
    'Cho\'Gath': 'Chogath',
    'Dr. Mundo': 'DrMundo',
    'Jarvan IV': 'JarvanIV',
    'Kai\'Sa': 'Kaisa',
    'Kha\'Zix': 'Khazix',
    'Kog\'Maw': 'KogMaw',
    'LeBlanc': 'Leblanc',
    'Lee Sin': 'LeeSin',
    'Master Yi': 'MasterYi',
    'Miss Fortune': 'MissFortune',
    'Nunu & Willump': 'Nunu',
    'Rek\'Sai': 'RekSai',
    'Tahm Kench': 'TahmKench',
    'Twisted Fate': 'TwistedFate',
    'Vel\'Koz': 'Velkoz',
    'Xin Zhao': 'XinZhao'
};

// Function to get the correct API name for a champion
function getApiName(championName) {
    return championNameMappings[championName] || championName.replace(/\s+/g, '').replace(/'/g, '').replace(/\./g, '').replace(/&/g, '').replace(/-/g, '');
}

// Function to add images to a champion object
function addImagesToChampion(championName) {
    const apiName = getApiName(championName);
    return {
        square: `https://ddragon.leagueoflegends.com/cdn/14.1.1/img/champion/${apiName}.png`,
        splash: `https://ddragon.leagueoflegends.com/cdn/img/champion/splash/${apiName}_0.jpg`,
        loading: `https://ddragon.leagueoflegends.com/cdn/img/champion/loading/${apiName}_0.jpg`
    };
}

// Example usage:
console.log('Ahri images:', addImagesToChampion('Ahri'));
console.log('Master Yi images:', addImagesToChampion('Master Yi'));
console.log('Kai\'Sa images:', addImagesToChampion('Kai\'Sa'));

// Function to generate image property for champion database
function generateImageProperty(championName) {
    const images = addImagesToChampion(championName);
    return `image: {
    square: '${images.square}',
    splash: '${images.splash}'
},`;
}

// List of all 173+ champions (you can use this to batch generate images)
const allChampionNames = [
    'Aatrox', 'Ahri', 'Akali', 'Akshan', 'Alistar', 'Ammu', 'Anivia', 'Annie', 'Aphelios', 'Ashe',
    'Aurelion Sol', 'Azir', 'Bard', 'Bel\'Veth', 'Blitzcrank', 'Brand', 'Braum', 'Caitlyn', 'Camille', 'Cassiopeia',
    'Cho\'Gath', 'Corki', 'Darius', 'Diana', 'Dr. Mundo', 'Draven', 'Ekko', 'Elise', 'Evelynn', 'Ezreal',
    'Fiddlesticks', 'Fiora', 'Fizz', 'Galio', 'Gangplank', 'Garen', 'Gnar', 'Gragas', 'Graves', 'Gwen',
    'Hecarim', 'Heimerdinger', 'Illaoi', 'Irelia', 'Ivern', 'Janna', 'Jarvan IV', 'Jax', 'Jayce', 'Jhin',
    'Jinx', 'Kai\'Sa', 'Kalista', 'Karma', 'Karthus', 'Kassadin', 'Katarina', 'Kayle', 'Kayn', 'Kennen',
    'Kha\'Zix', 'Kindred', 'Kled', 'Kog\'Maw', 'LeBlanc', 'Lee Sin', 'Leona', 'Lillia', 'Lissandra', 'Lucian',
    'Lulu', 'Lux', 'Malphite', 'Malzahar', 'Maokai', 'Master Yi', 'Miss Fortune', 'Mordekaiser', 'Morgana', 'Nami',
    'Nasus', 'Nautilus', 'Neeko', 'Nidalee', 'Nilah', 'Nocturne', 'Nunu & Willump', 'Olaf', 'Orianna', 'Ornn',
    'Pantheon', 'Poppy', 'Pyke', 'Qiyana', 'Quinn', 'Rakan', 'Rammus', 'Rek\'Sai', 'Rell', 'Renata Glasc',
    'Renekton', 'Rengar', 'Riven', 'Rumble', 'Ryze', 'Samira', 'Sejuani', 'Senna', 'Seraphine', 'Sett',
    'Shaco', 'Shen', 'Shyvana', 'Singed', 'Sion', 'Sivir', 'Skarner', 'Sona', 'Soraka', 'Swain',
    'Sylas', 'Syndra', 'Tahm Kench', 'Taliyah', 'Talon', 'Taric', 'Teemo', 'Thresh', 'Tristana', 'Trundle',
    'Tryndamere', 'Twisted Fate', 'Twitch', 'Udyr', 'Urgot', 'Varus', 'Vayne', 'Veigar', 'Vel\'Koz', 'Vex',
    'Vi', 'Viego', 'Viktor', 'Vladimir', 'Volibear', 'Warwick', 'Wukong', 'Xayah', 'Xerath', 'Xin Zhao',
    'Yasuo', 'Yone', 'Yorick', 'Yuumi', 'Zac', 'Zed', 'Zeri', 'Ziggs', 'Zilean', 'Zoe', 'Zyra'
];

// Generate images for all champions
console.log('\n=== Batch Image Generation ===');
allChampionNames.forEach(name => {
    console.log(`'${name}': { ..., ${generateImageProperty(name)} },`);
});

module.exports = {
    addImagesToChampion,
    generateImageProperty,
    getApiName,
    allChampionNames
};