use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 21v-6" ></ path > < path d = "M12 9V3" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < rect y = "3" rx = "2" height = "18" width = "18" x = "3" ></ rect > < / > } } pub const LUCIDE_TABLE_CELLS_MERGE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;