use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 15V9" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < rect height = "18" x = "3" y = "3" width = "18" rx = "2" ></ rect > < / > } } pub const LUCIDE_TABLE_CELLS_SPLIT : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none")] } ;