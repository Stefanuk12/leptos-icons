use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 5v4" ></ path > < rect x = "7" rx = "1" height = "6" y = "9" width = "4" ></ rect > < path d = "M9 15v2" ></ path > < path d = "M17 3v2" ></ path > < rect y = "5" height = "8" rx = "1" x = "15" width = "4" ></ rect > < path d = "M17 13v3" ></ path > < path d = "M3 3v18h18" ></ path > < / > } } pub const LUCIDE_CANDLESTICK_CHART : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;