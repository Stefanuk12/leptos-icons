use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 7h2a2 2 0 0 1 2 2v6c0 1-1 2-2 2h-2" ></ path > < path d = "M6 7H4a2 2 0 0 0-2 2v6c0 1 1 2 2 2h2" ></ path > < line y1 = "11" x2 = "22" x1 = "22" y2 = "13" ></ line > < line y1 = "7" x2 = "10" x1 = "10" y2 = "13" ></ line > < line x2 = "10" y2 = "17.01" x1 = "10" y1 = "17" ></ line > < / > } } pub const LUCIDE_BATTERY_WARNING : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;