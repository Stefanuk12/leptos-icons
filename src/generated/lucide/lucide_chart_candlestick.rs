use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 5v4" ></ path > < rect width = "4" y = "9" x = "7" height = "6" rx = "1" ></ rect > < path d = "M9 15v2" ></ path > < path d = "M17 3v2" ></ path > < rect height = "8" width = "4" x = "15" rx = "1" y = "5" ></ rect > < path d = "M17 13v3" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < / > } } pub const LUCIDE_CHART_CANDLESTICK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor")] } ;