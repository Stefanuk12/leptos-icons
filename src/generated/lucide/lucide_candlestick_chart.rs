use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 5v4" ></ path > < rect height = "6" x = "7" rx = "1" width = "4" y = "9" ></ rect > < path d = "M9 15v2" ></ path > < path d = "M17 3v2" ></ path > < rect x = "15" width = "4" y = "5" rx = "1" height = "8" ></ rect > < path d = "M17 13v3" ></ path > < path d = "M3 3v18h18" ></ path > < / > } } pub const LUCIDE_CANDLESTICK_CHART : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;