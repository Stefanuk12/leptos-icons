use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8" ></ path > < path d = "M7 14h.01" ></ path > < path d = "M17 14h.01" ></ path > < rect height = "8" width = "18" x = "3" y = "10" rx = "2" ></ rect > < path d = "M5 18v2" ></ path > < path d = "M19 18v2" ></ path > < / > } } pub const LUCIDE_CAR_FRONT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;