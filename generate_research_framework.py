import matplotlib.pyplot as plt
import matplotlib.patches as patches
from fpdf import FPDF
import datetime

def create_framework_diagram(filename='framework_diagram.png'):
    """Generates a detailed Research Framework Diagram using Matplotlib."""
    fig, ax = plt.subplots(figsize=(14, 8))
    ax.axis('off')
    
    # Common styles
    bbox_args = dict(boxstyle='round,pad=0.5', fc='white', ec='black', lw=1.5)
    arrow_args = dict(arrowstyle='->', lw=1.5, color='#455A64')
    
    def draw_box(x, y, width, height, color, text, title=None):
        rect = patches.FancyBboxPatch((x, y), width, height, boxstyle='round,pad=0.02', 
                                    facecolor=color, edgecolor='black', linewidth=1)
        ax.add_patch(rect)
        if title:
             ax.text(x + width/2, y + height - 0.05, title, fontsize=10, fontweight='bold', ha='center', va='top')
             ax.text(x + width/2, y + height/2 - 0.02, text, fontsize=9, ha='center', va='center')
        else:
             ax.text(x + width/2, y + height/2, text, fontsize=9, fontweight='bold', ha='center', va='center')

    # --- 1. DATA COLLECTION (Left) ---
    ax.text(0.1, 0.9, "1. DATA COLLECTION", fontsize=12, fontweight='bold', ha='center', color='#1565C0')
    draw_box(0.02, 0.4, 0.16, 0.4, '#BBDEFB', 
             "- User Questionnaire\n(10 Questions)\n\n"
             "- Behavioral Data\n(Playstyle/Role)\n\n"
             "- Psychological Data\n(Risk/Strategy)", "User Input")

    # --- 2. DATA PRE-PROCESSING (Middle-Left) ---
    ax.text(0.3, 0.9, "2. PRE-PROCESSING", fontsize=12, fontweight='bold', ha='center', color='#00695C')
    draw_box(0.22, 0.65, 0.16, 0.15, '#B2DFDB', "Feature Extraction\n(Mapping text to values)")
    draw_box(0.22, 0.45, 0.16, 0.15, '#B2DFDB', "Normalization\n(Scaling 0-10)")
    
    # Arrows 1->2
    ax.annotate('', xy=(0.22, 0.72), xytext=(0.18, 0.65), arrowprops=arrow_args)
    ax.annotate('', xy=(0.22, 0.52), xytext=(0.18, 0.55), arrowprops=arrow_args)

    # --- 3. ML Processing (Center) ---
    ax.text(0.55, 0.9, "3. ML ENSEMBLE ENGINE", fontsize=12, fontweight='bold', ha='center', color='#6A1B9A')
    # Container
    rect = patches.Rectangle((0.42, 0.35), 0.26, 0.5, linewidth=1, edgecolor='#7B1FA2', facecolor='#F3E5F5', linestyle='--')
    ax.add_patch(rect)
    
    draw_box(0.45, 0.72, 0.2, 0.08, '#E1BEE7', "Random Forest\n(Complex Rules)")
    draw_box(0.45, 0.56, 0.2, 0.08, '#E1BEE7', "Decision Tree\n(Hierarchical Logic)")
    draw_box(0.45, 0.40, 0.2, 0.08, '#E1BEE7', "K-Nearest Neighbors\n(Similarity Matching)")
    
    # Arrows 2->3
    ax.annotate('', xy=(0.45, 0.76), xytext=(0.38, 0.72), arrowprops=arrow_args)
    ax.annotate('', xy=(0.45, 0.60), xytext=(0.38, 0.52), arrowprops=arrow_args)
    ax.annotate('', xy=(0.45, 0.44), xytext=(0.38, 0.52), arrowprops=arrow_args)

    # --- 4. AGGREGATION (Middle-Right) ---
    ax.text(0.75, 0.9, "4. AGGREGATION", fontsize=12, fontweight='bold', ha='center', color='#EF6C00')
    draw_box(0.72, 0.5, 0.14, 0.2, '#FFE0B2', "Weighted Voting\n\nRF: 40%\nDT: 30%\nKNN: 30%", "Ensemble Logic")
    
    # Arrows 3->4
    ax.annotate('', xy=(0.72, 0.60), xytext=(0.65, 0.76), arrowprops=arrow_args)
    ax.annotate('', xy=(0.72, 0.60), xytext=(0.65, 0.60), arrowprops=arrow_args)
    ax.annotate('', xy=(0.72, 0.60), xytext=(0.65, 0.44), arrowprops=arrow_args)

    # --- 5. OUTPUT (Right) ---
    ax.text(0.92, 0.9, "5. RESULT", fontsize=12, fontweight='bold', ha='center', color='#2E7D32')
    draw_box(0.90, 0.4, 0.08, 0.4, '#C8E6C9', "Top 5\nChampions\n\n+\n\nMatch\nScores", "Output")

    # Arrow 4->5
    ax.annotate('', xy=(0.90, 0.60), xytext=(0.86, 0.60), arrowprops=arrow_args)

    plt.tight_layout()
    plt.savefig(filename, dpi=300, bbox_inches='tight')
    print(f"Detailed Diagram saved to {filename}")

class ResearchFrameworkPDF(FPDF):
    def header(self):
        self.set_font('Arial', 'B', 15)
        title = 'Research Framework & Methodology'
        title_w = self.get_string_width(title) + 6
        doc_w = self.w
        self.set_x((doc_w - title_w) / 2)
        self.cell(title_w, 10, title, border=0, ln=1, align='C')
        self.ln(10)

    def footer(self):
        self.set_y(-15)
        self.set_font('Arial', 'I', 8)
        self.cell(0, 10, 'Page ' + str(self.page_no()) + '/{nb}', 0, 0, 'C')

    def chapter_title(self, label):
        self.set_font('Arial', 'B', 12)
        self.set_fill_color(240, 240, 240)
        self.cell(0, 6, label, 0, 1, 'L', 1)
        self.ln(4)

    def chapter_body(self, body):
        self.set_font('Arial', '', 11)
        self.multi_cell(0, 5, body)
        self.ln()

def generate_report():
    # 1. Create Diagram
    diagram_filename = 'framework_diagram.png'
    create_framework_diagram(diagram_filename)

    # 2. Create PDF
    pdf = ResearchFrameworkPDF()
    pdf.alias_nb_pages()
    pdf.add_page()

    # --- Introduction ---
    pdf.chapter_title("1. Introduction Overview")
    pdf.chapter_body(
        "This report outlines the Research Framework designed for the League of Legends Champion Recommender System. "
        "The framework follows a systematic Input-Process-Output (IPO) model, demonstrating how user psychological data "
        "is transformed into personalized champion recommendations through machine learning techniques."
    )

    # --- Framework Diagram ---
    pdf.chapter_title("2. Research Framework Diagram")
    pdf.image(diagram_filename, x=10, w=190)
    pdf.ln(5)
    pdf.set_font('Arial', 'I', 9)
    pdf.cell(0, 5, "Figure 1: Input-Process-Output Model of the Recommender System", 0, 1, 'C')
    pdf.ln(10)

    # --- Detailed Explanation ---
    pdf.chapter_title("3. Framework Architecture Detail")
    
    # Introduction
    pdf.set_font('Arial', '', 11)
    pdf.multi_cell(0, 5, 
        "The research framework functions as a complex three-tiered pipeline effectively translating the subtleties of human communication into actionable digital data."
    )
    pdf.ln(5)

    # Input Phase
    pdf.set_font('Arial', 'B', 11)
    pdf.cell(0, 6, "A. Input Phase (Data Collection)", 0, 1)
    pdf.set_font('Arial', '', 11)
    pdf.multi_cell(0, 5, 
        "The Input Phase moves beyond simple statistics to build a multi-faceted 'Comprehensive User Profile'. "
        "Through a specialized questionnaire, the system captures two distinct data streams: explicit 'Behavioral Traits' "
        "(such as role preference or aggression levels) and implicit 'Psychological Traits' (measuring abstract qualities like risk tolerance and strategic foresight). "
        "These dimensions are synthesized into a holistic input, ensuring the model aligns the player's personality with their mechanical preferences."
    )
    pdf.ln(5)

    # Process Phase
    pdf.set_font('Arial', 'B', 11)
    pdf.cell(0, 6, "B. Process Phase (The Analytical Engine)", 0, 1)
    pdf.set_font('Arial', '', 11)
    pdf.multi_cell(0, 5, 
        "The Process Phase serves as the system's core intelligence. Instead of relying on a single algorithm, the framework employs an "
        "'Ensemble Machine Learning Architecture' acting as a computational 'Panel of Experts'. This engine harnesses the collective power of three models: "
        "a Random Forest classifier to identify complex, non-linear interactions; a Decision Tree for transparent, rule-based logic; and K-Nearest Neighbors (KNN) "
        "for mathematical similarity matching. This ensemble approach mitigates individual algorithmic bias and significantly enhances prediction accuracy."
    )
    pdf.ln(5)

    # Output Phase
    pdf.set_font('Arial', 'B', 11)
    pdf.cell(0, 6, "C. Output Phase (Actionable Intelligence)", 0, 1)
    pdf.set_font('Arial', '', 11)
    pdf.multi_cell(0, 5, 
        "The Output Phase converts raw probabilities into 'Actionable Intelligence'. To guarantee relevance and variety, specific metrics such as Precision@K "
        "and Diversity are applied to filter and rank the results, producing a tailored list of 'Champion Recommendations'. "
        "Fundamentally, the framework is designed to facilitate a cyclical feedback loop; user interactions and ratings serve as recursive inputs, "
        "allowing the system to iteratively refine its understanding and improve performance over time."
    )
    pdf.ln(5)

    output_filename = 'Research_Framework.pdf'
    pdf.output(output_filename, 'F')
    print(f"Successfully generated report: {output_filename}")

if __name__ == '__main__':
    generate_report()
