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
    private ArrayList<Light> lightList;

    public PaintPanel() {
        circleList = new ArrayList<>();
        lightList = new ArrayList<>();  
        this.setBorder(new LineBorder(Color.BLACK));
    }

    public void addCircle(Circle newC) {
        circleList.add(newC);
    }

    public void addLight(Light l) {
        lightList.add(l);
    }

    @Override
    protected void paintComponent(Graphics g) {
        super.paintComponent(g);

        for (int i = 0; i < circleList.size(); i++) {
            g.setColor(circleList.get(i).getColor());
            g.fillOval((int) (this.getWidth() / 2 + circleList.get(i).getX() * 50 - ((circleList.get(i).getRadius() * 50) / 2)),(int) (this.getHeight() / 2 - circleList.get(i).getY() * 50 + (circleList.get(i).getRadius() * 50) / 2),(int) ((circleList.get(i).getRadius()+0.01) * 100),(int) ((circleList.get(i).getRadius()+0.01) * 100));
        }
    }

    public void saveToFile() throws IOException {
        File inf = new File("inf.txt");
        BufferedWriter writer = new BufferedWriter(new FileWriter(inf));
        for (int i = 0; i < circleList.size(); i++) {
            writer.write(circleList.get(i).toString()+"\n");
        }

        if(lightList.size() == 0){
            System.out.println("No lights have been added!");
        }
        for (int i = 0; i < lightList.size(); i++) {
            writer.write(lightList.get(i).toString() + "\n");
        }

        writer.close();
    }

}
