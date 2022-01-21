import java.awt.Color;

/**
 *
 * @author ozkar
 */
public class Light {

    private double x, y, z;

    private Color color;

    private double intensity;

    public Light(double x, double y, double z, Color color, double intensity) {
        this.x = x;
        this.y = y;
        this.z = z;
        this.color = color;
        this.intensity = intensity/100;

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

    public double getZ() {
        return z;
    }

    public void setZ(double z) {
        this.z = z;
    }

    public Color getColor() {
        return color;
    }

    public void setColor(Color color) {
        this.color = color;
    }

    public double getIntensity() {
        return intensity;
    }

    public void setIntensity(double intensity) {
        this.intensity = intensity;
    }

    @Override
    public String toString() {
        return "light" + " " + x + " " + y + " " + z + " "+ intensity +  " " + color.getRed() + " " + color.getGreen() + " " + color.getBlue();
    }

}
