use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < rect width = "12" x = "7" y = "5" height = "4" rx = "1" ></ rect > < rect width = "7" rx = "1" x = "7" height = "4" y = "13" ></ rect > < / > } } pub const LUCIDE_BAR_CHART_HORIZONTAL_BIG : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;