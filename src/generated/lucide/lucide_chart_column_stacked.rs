use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 13H7" ></ path > < path d = "M19 9h-4" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect y = "5" x = "15" width = "4" rx = "1" height = "12" ></ rect > < rect height = "9" rx = "1" y = "8" x = "7" width = "4" ></ rect > < / > } } pub const LUCIDE_CHART_COLUMN_STACKED : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2")] } ;