use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 13H7" ></ path > < path d = "M19 9h-4" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < rect x = "15" rx = "1" y = "5" width = "4" height = "12" ></ rect > < rect rx = "1" y = "8" width = "4" x = "7" height = "9" ></ rect > < / > } } pub const LUCIDE_CHART_COLUMN_STACKED : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;