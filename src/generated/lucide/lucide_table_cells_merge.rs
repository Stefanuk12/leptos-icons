use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 21v-6" ></ path > < path d = "M12 9V3" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < rect height = "18" y = "3" x = "3" rx = "2" width = "18" ></ rect > < / > } } pub const LUCIDE_TABLE_CELLS_MERGE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24")] } ;