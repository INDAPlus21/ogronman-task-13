import java.awt.Color;

/**
 *
 * @author ozkar
 */
public class Circle {
    
    private double x;
    private double y;
    private double z;

    public double getZ() {
        return z;
    }

    public void setZ(double z) {
        this.z = z;
    }
    
    private Color color;
    
    private double radius;

    public Circle(double x, double y, double z, Color color, double radius) {
        this.x = x;
        this.y = y;
        this.z = z;
        this.color = color;
        this.radius = radius/100;

    }

    
    
    public double getX() {
        return x;
    }

    public void setX(double x) {
        this.x = x;
    }

    public double getY() {
        return y;
    }

    public void setY(double y) {
        this.y = y;
    }

    public Color getColor() {
        return color;
    }

    public void setColor(Color color) {
        this.color = color;
    }

    public double getRadius() {
        return radius;
    }

    public void setRadius(double radius) {
        this.radius = radius;
    }

    @Override
    public String toString() {        
        return  "sphere" + " " + x + " " + y + " " + z + " " + radius + " " + color.getRed() + " " + color.getGreen() + " " + color.getBlue() ;
    }
    
    
    
}
