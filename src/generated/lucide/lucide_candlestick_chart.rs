use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 5v4" ></ path > < rect width = "4" y = "9" x = "7" rx = "1" height = "6" ></ rect > < path d = "M9 15v2" ></ path > < path d = "M17 3v2" ></ path > < rect y = "5" x = "15" width = "4" rx = "1" height = "8" ></ rect > < path d = "M17 13v3" ></ path > < path d = "M3 3v18h18" ></ path > < / > } } pub const LUCIDE_CANDLESTICK_CHART : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("width" , "24")] } ;