use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < rect width = "4" height = "7" rx = "1" x = "7" y = "10" ></ rect > < rect height = "12" x = "15" rx = "1" y = "5" width = "4" ></ rect > < / > } } pub const LUCIDE_BAR_CHART_BIG : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;