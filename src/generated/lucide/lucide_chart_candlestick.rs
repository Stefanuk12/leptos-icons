use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 5v4" ></ path > < rect height = "6" y = "9" rx = "1" width = "4" x = "7" ></ rect > < path d = "M9 15v2" ></ path > < path d = "M17 3v2" ></ path > < rect height = "8" width = "4" rx = "1" x = "15" y = "5" ></ rect > < path d = "M17 13v3" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < / > } } pub const LUCIDE_CHART_CANDLESTICK : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;