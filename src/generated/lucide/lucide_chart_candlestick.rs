use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 5v4" ></ path > < rect width = "4" x = "7" y = "9" height = "6" rx = "1" ></ rect > < path d = "M9 15v2" ></ path > < path d = "M17 3v2" ></ path > < rect height = "8" x = "15" width = "4" y = "5" rx = "1" ></ rect > < path d = "M17 13v3" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < / > } } pub const LUCIDE_CHART_CANDLESTICK : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none")] } ;