use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 13H7" ></ path > < path d = "M19 9h-4" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect width = "4" height = "12" x = "15" y = "5" rx = "1" ></ rect > < rect width = "4" y = "8" x = "7" height = "9" rx = "1" ></ rect > < / > } } pub const LUCIDE_CHART_COLUMN_STACKED : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none")] } ;