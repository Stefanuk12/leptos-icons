use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8" ></ path > < path d = "M7 14h.01" ></ path > < path d = "M17 14h.01" ></ path > < rect height = "8" width = "18" rx = "2" x = "3" y = "10" ></ rect > < path d = "M5 18v2" ></ path > < path d = "M19 18v2" ></ path > < / > } } pub const LUCIDE_CAR_FRONT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;