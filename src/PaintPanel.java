import java.awt.Color;
import java.awt.Graphics;
import java.io.BufferedWriter;
import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.util.ArrayList;
import javax.swing.JPanel;
import javax.swing.border.LineBorder;

/**
 *
 * @author ozkar
 */
public class PaintPanel extends JPanel {

    private ArrayList<Circle> circleList;

    public PaintPanel() {
        circleList = new ArrayList<>();
        this.setBorder(new LineBorder(Color.BLACK));
    }

    public void addCircle(Circle newC) {
        circleList.add(newC);
    }

    @Override
    protected void paintComponent(Graphics g) {
        super.paintComponent(g);

        for (int i = 0; i < circleList.size(); i++) {
            g.setColor(circleList.get(i).getColor());
            g.fillOval(this.getWidth() / 2 + circleList.get(i).getX() * 50 - (circleList.get(i).getRadius() * 50) / 2, this.getHeight() / 2 - circleList.get(i).getY() * 50 + (circleList.get(i).getRadius() * 50) / 2, circleList.get(i).getRadius() * 100, circleList.get(i).getRadius() * 100);
        }
    }

    public void saveToFile() throws IOException {
        File inf = new File("inf.txt");
        BufferedWriter writer = new BufferedWriter(new FileWriter(inf));
        for (int i = 0; i < circleList.size(); i++) {
            writer.write(circleList.get(i).toString()+"\n");
        }
        writer.close();
    }

}
