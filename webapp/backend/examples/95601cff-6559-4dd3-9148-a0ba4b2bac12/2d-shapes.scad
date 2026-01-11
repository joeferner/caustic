// OpenSCAD 2D Shapes Demonstration
// This file shows all basic 2D primitives and operations

// Set the view to show 2D shapes properly
$fn = 50; // Smooth curves

spacing = 60; // Space between shapes

// Row 1: Basic Shapes
translate([0, 0, 0]) {
    // Circle
    translate([0, 0, 0]) {
        circle(r=20);
        translate([0, -35, 0]) 
            text("circle(r=20)", size=5, halign="center");
    }
    
    // Square
    translate([spacing, 0, 0]) {
        square([30, 30], center=true);
        translate([0, -35, 0]) 
            text("square([30,30])", size=5, halign="center");
    }
    
    // Rectangle
    translate([spacing*2, 0, 0]) {
        square([40, 25], center=true);
        translate([0, -35, 0]) 
            text("square([40,25])", size=5, halign="center");
    }
}

// Row 2: Polygon shapes
translate([0, -80, 0]) {
    // Triangle (polygon)
    translate([0, 0, 0]) {
        polygon(points=[[0,20], [-17,-10], [17,-10]]);
        translate([0, -35, 0]) 
            text("polygon - triangle", size=5, halign="center");
    }
    
    // Pentagon (polygon)
    translate([spacing, 0, 0]) {
        polygon(points=[
            [0, 20],
            [19, 6],
            [12, -16],
            [-12, -16],
            [-19, 6]
        ]);
        translate([0, -35, 0]) 
            text("polygon - pentagon", size=5, halign="center");
    }
    
    // Star (polygon)
    translate([spacing*2, 0, 0]) {
        polygon(points=[
            [0,20], [5,8], [18,8], [8,0], [12,-12],
            [0,-5], [-12,-12], [-8,0], [-18,8], [-5,8]
        ]);
        translate([0, -35, 0]) 
            text("polygon - star", size=5, halign="center");
    }
}

// Row 3: Text and Import
translate([0, -160, 0]) {
    // Text
    translate([0, 0, 0]) {
        text("Hello", size=15, halign="center", valign="center");
        translate([0, -35, 0]) 
            text("text()", size=5, halign="center");
    }
    
    // Circle with different $fn values
    translate([spacing, 0, 0]) {
        circle(r=20, $fn=6); // Hexagon
        translate([0, -35, 0]) 
            text("circle($fn=6)", size=5, halign="center");
    }
    
    // Ellipse (using scale)
    translate([spacing*2, 0, 0]) {
        scale([1.5, 1])
            circle(r=15);
        translate([0, -35, 0]) 
            text("scaled circle", size=5, halign="center");
    }
}

// Row 4: Boolean Operations
translate([0, -240, 0]) {
    // Union (default)
    translate([0, 0, 0]) {
        union() {
            circle(r=15);
            translate([12, 0, 0]) circle(r=15);
        }
        translate([0, -35, 0]) 
            text("union()", size=5, halign="center");
    }
    
    // Difference
    translate([spacing, 0, 0]) {
        difference() {
            circle(r=18);
            translate([8, 0, 0]) circle(r=12);
        }
        translate([0, -35, 0]) 
            text("difference()", size=5, halign="center");
    }
    
    // Intersection
    translate([spacing*2, 0, 0]) {
        intersection() {
            circle(r=15);
            translate([12, 0, 0]) circle(r=15);
        }
        translate([0, -35, 0]) 
            text("intersection()", size=5, halign="center");
    }
}

// Row 5: Transformations
translate([0, -320, 0]) {
    // Offset (outward)
    translate([0, 0, 0]) {
        offset(r=3) square([25, 25], center=true);
        translate([0, -35, 0]) 
            text("offset(r=3)", size=5, halign="center");
    }
    
    // Offset (inward)
    translate([spacing, 0, 0]) {
        offset(r=-3) square([30, 30], center=true);
        translate([0, -35, 0]) 
            text("offset(r=-3)", size=5, halign="center");
    }
    
    // Hull
    translate([spacing*2, 0, 0]) {
        hull() {
            translate([-10, -10, 0]) circle(r=5);
            translate([10, -10, 0]) circle(r=5);
            translate([0, 15, 0]) circle(r=5);
        }
        translate([0, -35, 0]) 
            text("hull()", size=5, halign="center");
    }
}

// Row 6: Advanced polygon with paths
translate([0, -400, 0]) {
    // Polygon with hole
    translate([0, 0, 0]) {
        polygon(
            points=[
                // Outer square
                [-20,-20], [20,-20], [20,20], [-20,20],
                // Inner square (hole)
                [-10,-10], [-10,10], [10,10], [10,-10]
            ],
            paths=[[0,1,2,3], [4,5,6,7]]
        );
        translate([0, -35, 0]) 
            text("polygon w/ hole", size=5, halign="center");
    }
    
    // Minkowski sum
    translate([spacing, 0, 0]) {
        minkowski() {
            square([20, 20], center=true);
            circle(r=3);
        }
        translate([0, -35, 0]) 
            text("minkowski()", size=5, halign="center");
    }
}
