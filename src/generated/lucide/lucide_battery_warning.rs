use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 7h2a2 2 0 0 1 2 2v6c0 1-1 2-2 2h-2" ></ path > < path d = "M6 7H4a2 2 0 0 0-2 2v6c0 1 1 2 2 2h2" ></ path > < line x1 = "22" x2 = "22" y2 = "13" y1 = "11" ></ line > < line x1 = "10" y1 = "7" y2 = "13" x2 = "10" ></ line > < line y1 = "17" x2 = "10" x1 = "10" y2 = "17.01" ></ line > < / > } } pub const LUCIDE_BATTERY_WARNING : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;