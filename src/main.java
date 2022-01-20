import java.io.IOException;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.File;
import java.nio.file.Path;
import java.io.File;
import java.io.UnsupportedEncodingException;
import java.net.URLDecoder;
import java.nio.file.Path;
import java.util.Map;
import java.util.List;
import java.awt.BorderLayout;
import java.awt.Color;
import java.awt.Dimension;
import java.awt.GridLayout;
import java.awt.HeadlessException;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.io.IOException;
import java.util.logging.Level;
import java.util.logging.Logger;
import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JPanel;
import javax.swing.JSlider;
import javax.swing.JTextField;
import javax.swing.border.LineBorder;
import javax.swing.event.ChangeEvent;
import javax.swing.event.ChangeListener;

public class main extends JFrame{

    private static native void hello();

    private static native void javaCall();

    private JSlider colorSliderR;
    private JSlider colorSliderG;
    private JSlider colorSliderB;
    private JSlider radiusSlider;
    private JTextField xPos;
    private JTextField yPos;
    private JTextField zPos;
    private JButton addCircle;
    private JButton colorButton;

    private Color currentColor;
    
    private JButton renderScene;

    private JPanel settingsPanel;


    public main() throws HeadlessException {

        settingsPanel = new JPanel();

        settingsPanel.setBorder(new LineBorder(Color.BLACK));

        initJComponents();

        this.setLayout(new BorderLayout());

        this.setPreferredSize(new Dimension(1000, 800));

        PaintPanel p = new PaintPanel();

        add(p, BorderLayout.CENTER);

        add(settingsPanel, BorderLayout.WEST);
        
        renderScene = new JButton("render scene");
        
        renderScene.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                try {
                    p.saveToFile();
                    main.javaCall();
                } catch (IOException ex) {
                    
                }
            }
        });
        
        add(renderScene, BorderLayout.EAST);

        addCircle.addActionListener(new ActionListener() {
            @Override
            public void actionPerformed(ActionEvent e) {
                try {
                    p.addCircle(new Circle(Integer.parseInt(xPos.getText()), Integer.parseInt(yPos.getText()), Integer.parseInt(zPos.getText()), currentColor, radiusSlider.getValue()));
                    p.repaint();
                } catch (java.lang.NumberFormatException er) {

                }
            }
        });

        colorSliderR.addChangeListener(new ChangeListener() {
            @Override
            public void stateChanged(ChangeEvent e) {
                currentColor = new Color(colorSliderR.getValue(), colorSliderG.getValue(), colorSliderB.getValue());
                colorButton.setBackground(currentColor);
                colorButton.setForeground(currentColor);
            }
        });
        colorSliderG.addChangeListener(new ChangeListener() {
            @Override
            public void stateChanged(ChangeEvent e) {
                currentColor = new Color(colorSliderR.getValue(), colorSliderG.getValue(), colorSliderB.getValue());
                colorButton.setBackground(currentColor);
                colorButton.setForeground(currentColor);
            }
        });
        colorSliderB.addChangeListener(new ChangeListener() {
            @Override
            public void stateChanged(ChangeEvent e) {
                currentColor = new Color(colorSliderR.getValue(), colorSliderG.getValue(), colorSliderB.getValue());
                colorButton.setBackground(currentColor);
                colorButton.setForeground(currentColor);
            }
        });

        this.pack();
        this.setVisible(true);
        this.setDefaultCloseOperation(EXIT_ON_CLOSE);
    }

    public void initJComponents() {
        colorSliderR = new JSlider();
        colorSliderR.setMaximum(255);
        colorSliderR.setMinimum(0);

        colorSliderG = new JSlider();
        colorSliderG.setMaximum(255);
        colorSliderG.setMinimum(0);

        colorSliderB = new JSlider();
        colorSliderB.setMaximum(255);
        colorSliderB.setMinimum(0);

        currentColor = new Color(colorSliderR.getValue(), colorSliderG.getValue(), colorSliderB.getValue());

        colorButton = new JButton();
        colorButton.setEnabled(false);
        colorButton.setBackground(currentColor);
        colorButton.setForeground(currentColor);

        radiusSlider = new JSlider();
        radiusSlider.setMaximum(5);
        radiusSlider.setMinimum(0);

        xPos = new JTextField();
        yPos = new JTextField();
        zPos = new JTextField();

        addCircle = new JButton("Add sphere to image");

        settingsPanel.setLayout(new GridLayout(18, 1));

        settingsPanel.add(new JLabel("Current color:"));
        settingsPanel.add(colorButton);
        settingsPanel.add(new JLabel("Red-value:"));
        settingsPanel.add(colorSliderR);
        settingsPanel.add(new JLabel("Green-value:"));
        settingsPanel.add(colorSliderG);
        settingsPanel.add(new JLabel("Blue-value:"));
        settingsPanel.add(colorSliderB);
        settingsPanel.add(new JLabel("Radius:"));
        settingsPanel.add(radiusSlider);
        settingsPanel.add(new JLabel("x-posistion"));
        settingsPanel.add(xPos);
        settingsPanel.add(new JLabel("y-posistion:"));
        settingsPanel.add(yPos);
        settingsPanel.add(new JLabel("z-posistion:"));
        settingsPanel.add(zPos);
        settingsPanel.add(new JLabel("Add sphere to image:"));
        settingsPanel.add(addCircle);

    }

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) throws IOException, InterruptedException {


        String path = main.class.getProtectionDomain().getCodeSource().getLocation().getPath();

        String decodedPath = URLDecoder.decode(path, "UTF-8");
        
        decodedPath = decodedPath.substring(0, decodedPath.length() - 4);

        decodedPath += "target/release/";

        if (System.getProperty("os.name").equals("Windows 10")){
            decodedPath += "embed.dll";
        }else if(System.getProperty("os.name").equals("Linux")){
            decodedPath += "libembed.so";
        }

        System.load(decodedPath);

        System.out.println(decodedPath);

        main m = new main();
 
    }
    
}