use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M19.07 4.93A10 10 0 0 0 6.99 3.34" ></ path > < path d = "M4 6h.01" ></ path > < path d = "M2.29 9.62A10 10 0 1 0 21.31 8.35" ></ path > < path d = "M16.24 7.76A6 6 0 1 0 8.23 16.67" ></ path > < path d = "M12 18h.01" ></ path > < path d = "M17.99 11.66A6 6 0 0 1 15.77 16.67" ></ path > < circle cy = "12" cx = "12" r = "2" ></ circle > < path d = "m13.41 10.59 5.66-5.66" ></ path > < / > } } pub const LUCIDE_RADAR : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;