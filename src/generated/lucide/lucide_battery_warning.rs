use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 7h2a2 2 0 0 1 2 2v6c0 1-1 2-2 2h-2" ></ path > < path d = "M6 7H4a2 2 0 0 0-2 2v6c0 1 1 2 2 2h2" ></ path > < line x1 = "22" y1 = "11" y2 = "13" x2 = "22" ></ line > < line x2 = "10" y1 = "7" y2 = "13" x1 = "10" ></ line > < line y1 = "17" y2 = "17.01" x2 = "10" x1 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_WARNING : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;