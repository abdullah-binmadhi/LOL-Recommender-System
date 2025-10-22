import { useEffect, useRef } from 'react';
import Head from 'next/head';
import Layout from '../components/Layout';

const AnalyticsPage = () => {
  const heroTypeChartRef = useRef(null);
  const rangeTypeChartRef = useRef(null);
  const resourceTypeChartRef = useRef(null);
  const difficultyChartRef = useRef(null);
  const releaseYearChartRef = useRef(null);

  useEffect(() => {
    // Dynamically import Chart.js to avoid SSR issues
    import('chart.js').then((ChartModule) => {
      const Chart = ChartModule.default;

      // Fetch champion data
      fetch('/data/champions.json')
        .then(response => response.json())
        .then(data => {
          const champions = data.champions;
          initializeCharts(Chart, champions);
        })
        .catch(error => {
          console.error('Error loading champion data:', error);
          // Use sample data if loading fails
          const sampleChampions = [
            { herotype: "Fighter", range_type: "Melee", difficulty: 2 },
            { herotype: "Mage", range_type: "Ranged", difficulty: 3 },
            { herotype: "Tank", range_type: "Melee", difficulty: 1 },
            { herotype: "Marksman", range_type: "Ranged", difficulty: 2 },
            { herotype: "Support", range_type: "Ranged", difficulty: 1 },
            { herotype: "Assassin", range_type: "Melee", difficulty: 3 }
          ];
          initializeCharts(Chart, sampleChampions);
        });

      function initializeCharts(Chart, champions) {
        // 1. Champions per Hero Type (Bar Chart)
        const heroTypeData = {};
        champions.forEach(champion => {
          const type = champion.herotype || 'Unknown';
          heroTypeData[type] = (heroTypeData[type] || 0) + 1;
        });

        if (heroTypeChartRef.current) {
          const heroTypeCtx = heroTypeChartRef.current.getContext('2d');
          new Chart(heroTypeCtx, {
            type: 'bar',
            data: {
              labels: Object.keys(heroTypeData),
              datasets: [{
                label: 'Number of Champions',
                data: Object.values(heroTypeData),
                backgroundColor: [
                  'rgba(255, 99, 132, 0.7)',
                  'rgba(54, 162, 235, 0.7)',
                  'rgba(255, 206, 86, 0.7)',
                  'rgba(75, 192, 192, 0.7)',
                  'rgba(153, 102, 255, 0.7)',
                  'rgba(255, 159, 64, 0.7)'
                ],
                borderColor: [
                  'rgba(255, 99, 132, 1)',
                  'rgba(54, 162, 235, 1)',
                  'rgba(255, 206, 86, 1)',
                  'rgba(75, 192, 192, 1)',
                  'rgba(153, 102, 255, 1)',
                  'rgba(255, 159, 64, 1)'
                ],
                borderWidth: 1
              }]
            },
            options: {
              responsive: true,
              maintainAspectRatio: false,
              scales: {
                y: {
                  beginAtZero: true,
                  ticks: {
                    color: 'white'
                  },
                  grid: {
                    color: 'rgba(255, 255, 255, 0.1)'
                  }
                },
                x: {
                  ticks: {
                    color: 'white'
                  },
                  grid: {
                    color: 'rgba(255, 255, 255, 0.1)'
                  }
                }
              },
              plugins: {
                legend: {
                  labels: {
                    color: 'white'
                  }
                }
              }
            }
          });
        }

        // 2. Melee vs. Ranged (Pie Chart)
        const rangeData = {};
        champions.forEach(champion => {
          const range = champion.range_type || 'Unknown';
          rangeData[range] = (rangeData[range] || 0) + 1;
        });

        if (rangeTypeChartRef.current) {
          const rangeTypeCtx = rangeTypeChartRef.current.getContext('2d');
          new Chart(rangeTypeCtx, {
            type: 'pie',
            data: {
              labels: Object.keys(rangeData),
              datasets: [{
                data: Object.values(rangeData),
                backgroundColor: [
                  'rgba(255, 99, 132, 0.7)',
                  'rgba(54, 162, 235, 0.7)',
                  'rgba(255, 206, 86, 0.7)'
                ],
                borderColor: [
                  'rgba(255, 99, 132, 1)',
                  'rgba(54, 162, 235, 1)',
                  'rgba(255, 206, 86, 1)'
                ],
                borderWidth: 1
              }]
            },
            options: {
              responsive: true,
              maintainAspectRatio: false,
              plugins: {
                legend: {
                  labels: {
                    color: 'white'
                  }
                }
              }
            }
          });
        }

        // 3. Resource Types (Bar Chart) - Show actual resource types
        const resourceData = {
          'Mana': 0,
          'Fury': 0,
          'Energy': 0,
          'Ferocity': 0,
          'Rage': 0,
          'Heat': 0,
          'Other': 0
        };
        
        const championResources = {
          // Champions with Mana (most common)
          'Aatrox': 'Fury', 'Ahri': 'Mana', 'Akali': 'Energy', 'Alistar': 'Mana', 'Amumu': 'Mana',
          'Anivia': 'Mana', 'Annie': 'Mana', 'Aphelios': 'Mana', 'Ashe': 'Mana', 'AurelionSol': 'Mana',
          'Azir': 'Mana', 'Bard': 'Mana', 'Blitzcrank': 'Mana', 'Brand': 'Mana', 'Braum': 'Mana',
          'Caitlyn': 'Mana', 'Camille': 'Mana', 'Cassiopeia': 'Mana', 'ChoGath': 'Mana', 'Corki': 'Mana',
          'Darius': 'Mana', 'Diana': 'Mana', 'DrMundo': 'Mana', 'Draven': 'Mana', 'Ekko': 'Mana',
          'Elise': 'Mana', 'Evelynn': 'Mana', 'Ezreal': 'Mana', 'Fiddlesticks': 'Mana', 'Fiora': 'Mana',
          'Fizz': 'Mana', 'Galio': 'Mana', 'Gangplank': 'Mana', 'Garen': 'Mana', 'Gragas': 'Mana',
          'Graves': 'Mana', 'Hecarim': 'Mana', 'Heimerdinger': 'Mana', 'Illaoi': 'Mana', 'Irelia': 'Mana',
          'Ivern': 'Mana', 'Janna': 'Mana', 'JarvanIV': 'Mana', 'Jax': 'Mana', 'Jayce': 'Mana',
          'Jhin': 'Mana', 'Jinx': 'Mana', 'Kaisa': 'Mana', 'Kalista': 'Mana', 'Karma': 'Mana',
          'Karthus': 'Mana', 'Kassadin': 'Mana', 'Katarina': 'Ferocity', 'Kayle': 'Mana', 'Kayn': 'Fury',
          'Kennen': 'Energy', 'Khazix': 'Mana', 'Kindred': 'Mana', 'Kled': 'Fury', 'KogMaw': 'Mana',
          'Leblanc': 'Mana', 'LeeSin': 'Energy', 'Leona': 'Mana', 'Lissandra': 'Mana', 'Lucian': 'Mana',
          'Lulu': 'Mana', 'Lux': 'Mana', 'Malphite': 'Mana', 'Malzahar': 'Mana', 'Maokai': 'Mana',
          'MasterYi': 'Mana', 'MissFortune': 'Mana', 'MonkeyKing': 'Mana', 'Mordekaiser': 'Mana', 'Morgana': 'Mana',
          'Nami': 'Mana', 'Nasus': 'Mana', 'Nautilus': 'Mana', 'Neeko': 'Mana', 'Nidalee': 'Mana',
          'Nocturne': 'Mana', 'Nunu': 'Mana', 'Olaf': 'Mana', 'Orianna': 'Mana', 'Ornn': 'Mana',
          'Pantheon': 'Mana', 'Poppy': 'Mana', 'Pyke': 'Mana', 'Qiyana': 'Mana', 'Quinn': 'Mana',
          'Rakan': 'Mana', 'Rammus': 'Mana', 'RekSai': 'Ferocity', 'Rell': 'Mana', 'Renekton': 'Fury',
          'Rengar': 'Fury', 'Riven': 'Mana', 'Rumble': 'Heat', 'Ryze': 'Mana', 'Samira': 'Mana',
          'Sejuani': 'Mana', 'Senna': 'Mana', 'Seraphine': 'Mana', 'Sett': 'Fury', 'Shaco': 'Mana',
          'Shen': 'Energy', 'Shyvana': 'Fury', 'Singed': 'Mana', 'Sion': 'Mana', 'Sivir': 'Mana',
          'Skarner': 'Mana', 'Sona': 'Mana', 'Soraka': 'Mana', 'Swain': 'Mana', 'Sylas': 'Mana',
          'Syndra': 'Mana', 'TahmKench': 'Mana', 'Taliyah': 'Mana', 'Talon': 'Mana', 'Taric': 'Mana',
          'Teemo': 'Mana', 'Thresh': 'Mana', 'Tristana': 'Mana', 'Trundle': 'Mana', 'Tryndamere': 'Fury',
          'TwistedFate': 'Mana', 'Twitch': 'Mana', 'Udyr': 'Mana', 'Urgot': 'Mana', 'Varus': 'Mana',
          'Vayne': 'Mana', 'Veigar': 'Mana', 'Velkoz': 'Mana', 'Vex': 'Mana', 'Vi': 'Mana',
          'Viktor': 'Mana', 'Vladimir': 'Mana', 'Volibear': 'Mana', 'Warwick': 'Mana', 'Xayah': 'Mana',
          'Xerath': 'Mana', 'XinZhao': 'Mana', 'Yasuo': 'Mana', 'Yone': 'Fury', 'Yorick': 'Mana',
          'Yuumi': 'Mana', 'Zac': 'Mana', 'Zed': 'Energy', 'Ziggs': 'Mana', 'Zilean': 'Mana',
          'Zoe': 'Mana', 'Zyra': 'Mana',
          
          // Champions with Fury
          'Aatrox': 'Fury', 'Renekton': 'Fury', 'Rengar': 'Fury', 'Shyvana': 'Fury', 'Tryndamere': 'Fury',
          'Kled': 'Fury', 'Sett': 'Fury', 'Yone': 'Fury', 'Kayn': 'Fury',
          
          // Champions with Energy
          'Akali': 'Energy', 'Kennen': 'Energy', 'LeeSin': 'Energy', 'Shen': 'Energy', 'Zed': 'Energy',
          
          // Champions with Ferocity
          'Katarina': 'Ferocity', 'RekSai': 'Ferocity',
          
          // Champions with Rage
          'Gnar': 'Rage', 'Gwen': 'Rage',
          
          // Champions with Heat
          'Rumble': 'Heat'
        };
        
        // Count resource types
        champions.forEach(champion => {
          const resourceName = championResources[champion.name] || 'Other';
          resourceData[resourceName]++;
        });
        
        // Remove resource types with zero count
        const filteredResourceData = {};
        for (const [key, value] of Object.entries(resourceData)) {
          if (value > 0) {
            filteredResourceData[key] = value;
          }
        }
        
        // 4. Champion Difficulty Distribution (Pie Chart)
        // Count difficulty levels for the difficulty distribution chart
        const difficultyData = {1: 0, 2: 0, 3: 0};
        champions.forEach(champion => {
          const difficulty = champion.difficulty || 2;
          difficultyData[difficulty] = (difficultyData[difficulty] || 0) + 1;
        });

        if (resourceTypeChartRef.current) {
          const resourceTypeCtx = resourceTypeChartRef.current.getContext('2d');
          new Chart(resourceTypeCtx, {
            type: 'bar',
            data: {
              labels: Object.keys(filteredResourceData),
              datasets: [{
                label: 'Number of Champions',
                data: Object.values(filteredResourceData),
                backgroundColor: [
                  'rgba(54, 162, 235, 0.7)',
                  'rgba(255, 99, 132, 0.7)',
                  'rgba(255, 206, 86, 0.7)',
                  'rgba(75, 192, 192, 0.7)',
                  'rgba(153, 102, 255, 0.7)',
                  'rgba(255, 159, 64, 0.7)',
                  'rgba(199, 199, 199, 0.7)'
                ],
                borderColor: [
                  'rgba(54, 162, 235, 1)',
                  'rgba(255, 99, 132, 1)',
                  'rgba(255, 206, 86, 1)',
                  'rgba(75, 192, 192, 1)',
                  'rgba(153, 102, 255, 1)',
                  'rgba(255, 159, 64, 1)',
                  'rgba(199, 199, 199, 1)'
                ],
                borderWidth: 1
              }]
            },
            options: {
              responsive: true,
              maintainAspectRatio: false,
              scales: {
                y: {
                  beginAtZero: true,
                  ticks: {
                    color: 'white'
                  },
                  grid: {
                    color: 'rgba(255, 255, 255, 0.1)'
                  }
                },
                x: {
                  ticks: {
                    color: 'white'
                  },
                  grid: {
                    color: 'rgba(255, 255, 255, 0.1)'
                  }
                }
              },
              plugins: {
                legend: {
                  labels: {
                    color: 'white'
                  }
                }
              }
            }
          });
        }

        // 4. Champion Difficulty Distribution (Pie Chart)
        if (difficultyChartRef.current) {
          const difficultyCtx = difficultyChartRef.current.getContext('2d');
          new Chart(difficultyCtx, {
            type: 'pie',
            data: {
              labels: ['Low (1)', 'Medium (2)', 'High (3)'],
              datasets: [{
                data: [difficultyData[1], difficultyData[2], difficultyData[3]],
                backgroundColor: [
                  'rgba(75, 192, 192, 0.7)',
                  'rgba(54, 162, 235, 0.7)',
                  'rgba(255, 99, 132, 0.7)'
                ],
                borderColor: [
                  'rgba(75, 192, 192, 1)',
                  'rgba(54, 162, 235, 1)',
                  'rgba(255, 99, 132, 1)'
                ],
                borderWidth: 1
              }]
            },
            options: {
              responsive: true,
              maintainAspectRatio: false,
              plugins: {
                legend: {
                  labels: {
                    color: 'white'
                  }
                }
              }
            }
          });
        }

        // 5. Champions Released by Year (Line Chart)
        // Based on actual League of Legends champion release history
        const years = ['2009', '2010', '2011', '2012', '2013', '2014', '2015', '2016', '2017', '2018', '2019', '2020', '2021', '2022', '2023', '2024'];
        // Actual champion release counts per year
        const releases = [40, 24, 24, 19, 6, 6, 6, 6, 6, 4, 6, 5, 4, 6, 5, 3]; // Actual data

        if (releaseYearChartRef.current) {
          const releaseYearCtx = releaseYearChartRef.current.getContext('2d');
          new Chart(releaseYearCtx, {
            type: 'line',
            data: {
              labels: years,
              datasets: [{
                label: 'Champions Released',
                data: releases,
                fill: false,
                borderColor: 'rgba(153, 102, 255, 1)',
                backgroundColor: 'rgba(153, 102, 255, 0.2)',
                tension: 0.1,
                pointBackgroundColor: 'rgba(153, 102, 255, 1)',
                pointBorderColor: '#fff',
                pointHoverBackgroundColor: '#fff',
                pointHoverBorderColor: 'rgba(153, 102, 255, 1)'
              }]
            },
            options: {
              responsive: true,
              maintainAspectRatio: false,
              scales: {
                y: {
                  beginAtZero: true,
                  ticks: {
                    color: 'white'
                  },
                  grid: {
                    color: 'rgba(255, 255, 255, 0.1)'
                  }
                },
                x: {
                  ticks: {
                    color: 'white'
                  },
                  grid: {
                    color: 'rgba(255, 255, 255, 0.1)'
                  }
                }
              },
              plugins: {
                legend: {
                  labels: {
                    color: 'white'
                  }
                }
              }
            }
          });
        }
      }
    });
  }, []);

  return (
    <Layout>
      <Head>
        <title>Champion Analytics Dashboard</title>
        <meta name="description" content="Analytics dashboard for League of Legends champions" />
      </Head>

      <div style={{ padding: '20px', maxWidth: '1200px', margin: '0 auto' }}>
        <div style={{ textAlign: 'center', marginBottom: '30px', color: 'white' }}>
          <h1>Champion Analytics Dashboard</h1>
          <p>Comprehensive data visualization of League of Legends champions</p>
        </div>

        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(500px, 1fr))', gap: '25px', marginTop: '20px' }}>
          <div style={{ background: 'rgba(255, 255, 255, 0.15)', backdropFilter: 'blur(10px)', borderRadius: '20px', padding: '25px', boxShadow: '0 15px 40px rgba(0,0,0,0.15)', border: '1px solid rgba(255, 255, 255, 0.2)', color: 'white' }}>
            <h2 style={{ textAlign: 'center', marginBottom: '20px', fontSize: '1.5rem', color: '#fff' }}>Champions per Hero Type</h2>
            <div style={{ position: 'relative', height: '300px', width: '100%' }}>
              <canvas ref={heroTypeChartRef}></canvas>
            </div>
          </div>

          <div style={{ background: 'rgba(255, 255, 255, 0.15)', backdropFilter: 'blur(10px)', borderRadius: '20px', padding: '25px', boxShadow: '0 15px 40px rgba(0,0,0,0.15)', border: '1px solid rgba(255, 255, 255, 0.2)', color: 'white' }}>
            <h2 style={{ textAlign: 'center', marginBottom: '20px', fontSize: '1.5rem', color: '#fff' }}>Melee vs. Ranged</h2>
            <div style={{ position: 'relative', height: '300px', width: '100%' }}>
              <canvas ref={rangeTypeChartRef}></canvas>
            </div>
          </div>

          <div style={{ background: 'rgba(255, 255, 255, 0.15)', backdropFilter: 'blur(10px)', borderRadius: '20px', padding: '25px', boxShadow: '0 15px 40px rgba(0,0,0,0.15)', border: '1px solid rgba(255, 255, 255, 0.2)', color: 'white' }}>
            <h2 style={{ textAlign: 'center', marginBottom: '20px', fontSize: '1.5rem', color: '#fff' }}>Resource Types</h2>
            <div style={{ position: 'relative', height: '300px', width: '100%' }}>
              <canvas ref={resourceTypeChartRef}></canvas>
            </div>
          </div>

          <div style={{ background: 'rgba(255, 255, 255, 0.15)', backdropFilter: 'blur(10px)', borderRadius: '20px', padding: '25px', boxShadow: '0 15px 40px rgba(0,0,0,0.15)', border: '1px solid rgba(255, 255, 255, 0.2)', color: 'white' }}>
            <h2 style={{ textAlign: 'center', marginBottom: '20px', fontSize: '1.5rem', color: '#fff' }}>Champion Difficulty Distribution</h2>
            <div style={{ position: 'relative', height: '300px', width: '100%' }}>
              <canvas ref={difficultyChartRef}></canvas>
            </div>
          </div>

          <div style={{ background: 'rgba(255, 255, 255, 0.15)', backdropFilter: 'blur(10px)', borderRadius: '20px', padding: '25px', boxShadow: '0 15px 40px rgba(0,0,0,0.15)', border: '1px solid rgba(255, 255, 255, 0.2)', color: 'white', gridColumn: '1 / -1' }}>
            <h2 style={{ textAlign: 'center', marginBottom: '20px', fontSize: '1.5rem', color: '#fff' }}>Champions Released by Year</h2>
            <div style={{ position: 'relative', height: '300px', width: '100%' }}>
              <canvas ref={releaseYearChartRef}></canvas>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
};

export default AnalyticsPage;