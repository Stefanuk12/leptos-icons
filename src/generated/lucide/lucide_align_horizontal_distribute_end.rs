use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "6" y = "5" height = "14" x = "4" ></ rect > < rect x = "14" rx = "2" width = "6" y = "7" height = "10" ></ rect > < path d = "M10 2v20" ></ path > < path d = "M20 2v20" ></ path > < / > } } pub const LucideAlignHorizontalDistributeEnd : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none")] } ;