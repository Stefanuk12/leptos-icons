use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12.75 7.09a3 3 0 0 1 2.16 2.16" ></ path > < path d = "M17.072 17.072c-1.634 2.17-3.527 3.912-4.471 4.727a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 1.432-4.568" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M8.475 2.818A8 8 0 0 1 20 10c0 1.183-.31 2.377-.81 3.533" ></ path > < path d = "M9.13 9.13a3 3 0 0 0 3.74 3.74" ></ path > < / > } } pub const LUCIDE_MAP_PIN_OFF : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;