use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11.7 3H5a2 2 0 0 0-2 2v16l4-4h12a2 2 0 0 0 2-2v-2.7" ></ path > < circle cx = "18" r = "3" cy = "6" ></ circle > < / > } } pub const LucideMessageSquareDot : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;