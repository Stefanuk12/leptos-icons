use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 21v-6" ></ path > < path d = "M12 9V3" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < rect rx = "2" width = "18" height = "18" y = "3" x = "3" ></ rect > < / > } } pub const LUCIDE_TABLE_CELLS_MERGE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24")] } ;