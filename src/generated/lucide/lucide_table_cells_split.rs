use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 15V9" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < rect rx = "2" width = "18" height = "18" y = "3" x = "3" ></ rect > < / > } } pub const LUCIDE_TABLE_CELLS_SPLIT : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;