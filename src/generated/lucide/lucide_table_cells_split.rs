use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 15V9" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < rect x = "3" width = "18" height = "18" y = "3" rx = "2" ></ rect > < / > } } pub const LUCIDE_TABLE_CELLS_SPLIT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;