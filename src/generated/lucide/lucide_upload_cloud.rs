use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242" ></ path > < path d = "M12 12v9" ></ path > < path d = "m16 16-4-4-4 4" ></ path > < / > } } pub const LucideUploadCloud : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;