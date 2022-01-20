import java.awt.Color;

/**
 *
 * @author ozkar
 */
public class Circle {
    
    private int x;
    private int y;
    private int z;

    public int getZ() {
        return z;
    }

    public void setZ(int z) {
        this.z = z;
    }
    
    private Color color;
    
    private int radius;

    public Circle(int x, int y, int z, Color color, int radius) {
        this.x = x;
        this.y = y;
        this.z = z;
        this.color = color;
        this.radius = radius;
    }

    
    
    public int getX() {
        return x;
    }

    public void setX(int x) {
        this.x = x;
    }

    public int getY() {
        return y;
    }

    public void setY(int y) {
        this.y = y;
    }

    public Color getColor() {
        return color;
    }

    public void setColor(Color color) {
        this.color = color;
    }

    public int getRadius() {
        return radius;
    }

    public void setRadius(int radius) {
        this.radius = radius;
    }

    @Override
    public String toString() {
        return  "sphere" + " " + x + " " + y + " " + z + " " + radius + " " + color.getRed() + " " + color.getGreen() + " " + color.getBlue() ;
    }
    
    
    
}
