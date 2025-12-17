from fpdf import FPDF
import os

class PDF(FPDF):
    def header(self):
        self.set_font('Arial', 'B', 15)
        self.cell(0, 10, 'Champion Analysis Report', 0, 1, 'C')
        self.ln(10)

    def footer(self):
        self.set_y(-15)
        self.set_font('Arial', 'I', 8)
        self.cell(0, 10, 'Page ' + str(self.page_no()) + '/{nb}', 0, 0, 'C')

    def chapter_title(self, title):
        self.set_font('Arial', 'B', 12)
        self.set_fill_color(200, 220, 255)
        self.cell(0, 6, title, 0, 1, 'L', 1)
        self.ln(4)

    def chapter_body(self, body):
        self.set_font('Arial', '', 11)
        self.multi_cell(0, 5, body)
        self.ln()

    def add_chart_section(self, title, image_path, explanation):
        self.add_page()
        self.chapter_title(title)
        
        # Add Image
        if os.path.exists(image_path):
            # Center image
            self.image(image_path, x=15, w=180)
            self.ln(5)
        else:
            self.cell(0, 10, f"Image not found: {image_path}", 0, 1)
            
        # Add Explanation
        self.ln(5)
        self.set_font('Arial', 'B', 11)
        self.cell(0, 10, 'Analysis:', 0, 1)
        self.set_font('Arial', '', 11)
        self.multi_cell(0, 5, explanation)
        self.ln(10)

def generate_pdf():
    pdf = PDF()
    pdf.alias_nb_pages()
    
    # Title Page
    pdf.add_page()
    pdf.set_font('Arial', 'B', 24)
    pdf.cell(0, 60, '', 0, 1)
    pdf.cell(0, 20, 'League of Legends', 0, 1, 'C')
    pdf.cell(0, 20, 'Champion Data Analysis', 0, 1, 'C')
    pdf.set_font('Arial', '', 14)
    pdf.cell(0, 20, 'December 17, 2025', 0, 1, 'C')
    
    # Executive Summary
    pdf.add_page()
    pdf.chapter_title('Executive Summary')
    summary = (
        "This report provides a comprehensive visual analysis of the League of Legends champion roster. "
        "By examining key attributes such as hero type, combat range, resource management, difficulty, "
        "and release history, we gain insights into the game's design philosophy and evolution.\n\n"
        "The data reveals a balanced ecosystem with a slight preference for damage-dealing roles (Fighters/Mages) "
        "and a near-even split between melee and ranged combat styles. Accessibility is a clear priority, "
        "with the majority of champions falling into the 'Medium' difficulty category. Furthermore, the "
        "release cadence has shifted significantly over time, moving from rapid expansion to a more "
        "measured, quality-focused approach in recent years."
    )
    pdf.chapter_body(summary)

    # 1. Hero Type Analysis
    pdf.add_chart_section(
        '1. Champions per Hero Type',
        'chart_hero_type.png',
        "The bar chart illustrates the distribution of champions across six primary roles. "
        "Fighters are the most populous class, accounting for the largest share of the roster. "
        "This suggests a design emphasis on versatile, skirmish-heavy champions that can adapt to multiple "
        "situations. Mages follow closely, highlighting the importance of ability-based damage dealers. "
        "Assassins and Supports represent the smallest groups, likely due to their more specialized "
        "gameplay niches which require careful balancing to prevent them from dominating the meta."
    )

    # 2. Melee vs Ranged
    pdf.add_chart_section(
        '2. Melee vs. Ranged Distribution',
        'chart_range_type.png',
        "This pie chart displays the fundamental combat range split within the game. "
        "The distribution is remarkably balanced, with Melee champions holding a slight majority over Ranged ones. "
        "This balance is critical for the game's strategic depth, ensuring that team compositions can "
        "effectively mix frontline durability (typically melee) with backline damage output (typically ranged). "
        "The near 50/50 split indicates a conscious effort by the developers to support diverse playstyles "
        "and prevent either combat style from becoming overwhelmingly dominant."
    )

    # 3. Resource Types
    pdf.add_chart_section(
        '3. Resource Type Distribution',
        'chart_resource_type.png',
        "This chart breaks down the various resources champions use to cast abilities. "
        "Mana is by far the most common resource, serving as the standard gating mechanism for ability usage. "
        "However, a significant number of champions use alternative systems like Energy, Rage, or are entirely "
        "resourceless (cooldown-gated). These alternative systems add variety to lane dynamics; for instance, "
        "Energy champions are often limited by rapid burst windows, while resourceless champions can sustain "
        "in lane indefinitely but are balanced by longer cooldowns or weaker base stats."
    )

    # 4. Difficulty
    pdf.add_chart_section(
        '4. Champion Difficulty Distribution',
        'chart_difficulty.png',
        "The difficulty distribution highlights the game's accessibility curve. "
        "The majority of champions are classified as 'Medium' difficulty, offering a balance between "
        "approachability and mastery depth. 'Low' difficulty champions make up a substantial portion, "
        "ensuring new players have plenty of options to learn the basics without being overwhelmed. "
        "High difficulty champions are the minority, reserved for complex mechanics that reward high "
        "skill expression. This bell-curve distribution ensures the game remains accessible to beginners "
        "while providing deep challenges for veterans."
    )

    # 5. Release History
    pdf.add_chart_section(
        '5. Champions Released by Year',
        'chart_release_year.png',
        "The line graph tracks the number of champion releases annually from 2009 to 2024. "
        "A clear trend is visible: the early years (2009-2012) saw an explosion of content with "
        "dozens of champions released per year to rapidly build the roster. Starting around 2013, "
        "the release cadence slowed dramatically to a steady 4-6 champions per year. This shift "
        "reflects a transition from a 'growth phase' to a 'maturity phase', where the focus moved "
        "from quantity to quality, reworking older champions, and ensuring new releases bring "
        "unique, game-changing mechanics rather than just filling roster slots."
    )

    # Save
    output_path = 'Champion_Analysis_Report.pdf'
    pdf.output(output_path, 'F')
    print(f"PDF generated successfully: {output_path}")

if __name__ == "__main__":
    generate_pdf()
