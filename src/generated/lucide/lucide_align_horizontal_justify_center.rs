use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" x = "2" rx = "2" width = "6" y = "5" ></ rect > < rect rx = "2" x = "16" height = "10" y = "7" width = "6" ></ rect > < path d = "M12 2v20" ></ path > < / > } } pub const LucideAlignHorizontalJustifyCenter : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round")] } ;