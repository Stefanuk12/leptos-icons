use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 21v-6" ></ path > < path d = "M12 9V3" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < rect x = "3" width = "18" rx = "2" y = "3" height = "18" ></ rect > < / > } } pub const LUCIDE_TABLE_CELLS_MERGE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;