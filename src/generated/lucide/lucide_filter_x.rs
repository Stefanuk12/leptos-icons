use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13.013 3H2l8 9.46V19l4 2v-8.54l.9-1.055" ></ path > < path d = "m22 3-5 5" ></ path > < path d = "m17 3 5 5" ></ path > < / > } } pub const LucideFilterX : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("height" , "24")] } ;