use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.3 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10.8" ></ path > < path d = "m21 15-3.1-3.1a2 2 0 0 0-2.814.014L6 21" ></ path > < path d = "m14 19.5 3 3v-6" ></ path > < path d = "m17 22.5 3-3" ></ path > < / > } } pub const LucideImageDown : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24")] } ;