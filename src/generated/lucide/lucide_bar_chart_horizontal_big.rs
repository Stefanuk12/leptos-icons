use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < rect width = "12" height = "4" y = "5" rx = "1" x = "7" ></ rect > < rect height = "4" y = "13" x = "7" width = "7" rx = "1" ></ rect > < / > } } pub const LUCIDE_BAR_CHART_HORIZONTAL_BIG : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;