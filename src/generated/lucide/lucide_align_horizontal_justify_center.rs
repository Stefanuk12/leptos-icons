use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" y = "5" width = "6" rx = "2" height = "14" ></ rect > < rect y = "7" rx = "2" x = "16" width = "6" height = "10" ></ rect > < path d = "M12 2v20" ></ path > < / > } } pub const LucideAlignHorizontalJustifyCenter : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;