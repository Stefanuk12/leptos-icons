use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 11h.01" ></ path > < path d = "M11 15h.01" ></ path > < path d = "M16 16h.01" ></ path > < path d = "m2 16 20 6-6-20A20 20 0 0 0 2 16" ></ path > < path d = "M5.71 17.11a17.04 17.04 0 0 1 11.4-11.4" ></ path > < / > } } pub const LUCIDE_PIZZA : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;