use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v18h18" ></ path > < rect width = "4" height = "7" y = "10" rx = "1" x = "7" ></ rect > < rect rx = "1" width = "4" x = "15" y = "5" height = "12" ></ rect > < / > } } pub const LUCIDE_BAR_CHART_BIG : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;