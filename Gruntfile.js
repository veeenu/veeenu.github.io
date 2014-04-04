module.exports = function(grunt) {

  grunt.initConfig({
    pkg: grunt.file.readJSON('package.json'),
    jade: {
      build: {
        files: grunt.file.expandMapping(['**/*.jade'], '', {
          cwd: 'assets/jade',
          rename: function(dB, dP) {
            return dB + dP.replace(/\.jade$/, '.html');
          }
        })
      }
    },
    sass: {
      build: {
        files: {
          'css/style.css': 'assets/scss/style.scss'
        },
        options: {
          style: 'compressed'
        }
      }
    },
    watch: {
      jade: {
        files: [ 'assets/jade/**/*.jade' ],
        tasks: [ 'jade:build' ]
      },
      sass: {
        files: [ 'assets/scss/**/*.scss' ],
        tasks: [ 'sass:build' ]
      }
    },
    shell: {
      jekyll: {
        command: 'jekyll serve --watch'
      }
    },
    concurrent: {
      server: ['shell:jekyll', 'watch'],
      options: {
        logConcurrentOutput: true
      }
    }
  });

  grunt.loadNpmTasks('grunt-contrib-jade');
  grunt.loadNpmTasks('grunt-contrib-sass');
  grunt.loadNpmTasks('grunt-contrib-watch');
  grunt.loadNpmTasks('grunt-concurrent');
  grunt.loadNpmTasks('grunt-shell');

  grunt.registerTask('default', ['concurrent:server']);
  grunt.registerTask('build', ['jade:build', 'sass:build']);

}
