use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect width = "4" y = "5" x = "15" height = "12" rx = "1" ></ rect > < rect rx = "1" height = "9" x = "7" y = "8" width = "4" ></ rect > < / > } } pub const LUCIDE_CHART_COLUMN_BIG : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;