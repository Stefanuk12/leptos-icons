use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "14" rx = "2" x = "4" y = "5" width = "6" ></ rect > < rect width = "6" height = "10" x = "14" y = "7" rx = "2" ></ rect > < path d = "M4 2v20" ></ path > < path d = "M14 2v20" ></ path > < / > } } pub const LucideAlignHorizontalDistributeStart : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;