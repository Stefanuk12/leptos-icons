use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 7h1a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2h-2" ></ path > < path d = "M6 7H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h1" ></ path > < path d = "m11 7-3 5h4l-3 5" ></ path > < line x1 = "22" x2 = "22" y2 = "13" y1 = "11" ></ line > < / > } } pub const LUCIDE_BATTERY_CHARGING : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2")] } ;