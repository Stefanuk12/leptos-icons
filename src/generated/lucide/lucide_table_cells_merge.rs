use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 21v-6" ></ path > < path d = "M12 9V3" ></ path > < path d = "M3 15h18" ></ path > < path d = "M3 9h18" ></ path > < rect x = "3" rx = "2" height = "18" width = "18" y = "3" ></ rect > < / > } } pub const LucideTableCellsMerge : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round")] } ;