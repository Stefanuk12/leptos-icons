use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "1" width = "20" height = "5" x = "2" ></ rect > < path d = "M4 8v11a2 2 0 0 0 2 2h2" ></ path > < path d = "M20 8v11a2 2 0 0 1-2 2h-2" ></ path > < path d = "m9 15 3-3 3 3" ></ path > < path d = "M12 12v9" ></ path > < / > } } pub const LUCIDE_ARCHIVE_RESTORE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;