use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 15V9" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < rect height = "18" width = "18" x = "3" rx = "2" y = "3" ></ rect > < / > } } pub const LUCIDE_TABLE_CELLS_SPLIT : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;