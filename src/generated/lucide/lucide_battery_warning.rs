use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 7h2a2 2 0 0 1 2 2v6c0 1-1 2-2 2h-2" ></ path > < path d = "M6 7H4a2 2 0 0 0-2 2v6c0 1 1 2 2 2h2" ></ path > < line x2 = "22" x1 = "22" y2 = "13" y1 = "11" ></ line > < line y1 = "7" y2 = "13" x1 = "10" x2 = "10" ></ line > < line y2 = "17.01" y1 = "17" x2 = "10" x1 = "10" ></ line > < / > } } pub const LUCIDE_BATTERY_WARNING : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;